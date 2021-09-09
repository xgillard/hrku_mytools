# My tools on heroku

La 1e chose à faire c'est de connecter son repo à l'app heroku. Normalement 
ils donne la procédure à suivre qd on crée l'appli. Mais si jamais, ça tourne
autour de ça: 

```bash
heroku login
heroku git:remote -a hrku-mytools
```

Après ça, chaque fois qu'on fera un `git push heroku main`, ça va déployer 
l'application. Pour que heroku compile le code rust lorsqu'on fait un push, il 
faut utiliser le buildpack rust pour heroku (https://github.com/emk/heroku-buildpack-rust). 
On l'active avec la commande: `heroku buildpacks:set emk/rust`

## Configuration de Rocket
De mon pdv, le plus simple pour faire du web en rust est d'utiliser Rocket.
Meme si rocket 0.5.x n'a besoin que d'un rustc stable, pour le moment la version
0.4.x qui est dispo sur crates.io nécessite un nightly. Perso je ne suis pas fan
de bosser non stop sur une nightly qu'il faut mettre à jour tout le temps; donc
je préfère rester sur une version stable du compilo. Ce qui veut dire que pour
pouvoir utiliser rocket, la 1e chose à faire est de pointer vers la version
courante du repository: 

```toml
rocket = {version = "0.5.0-rc.1", git = "https://github.com/SergioBenitez/Rocket"}
```

## Lancer le server
Comme heroku ne sait a priori pas ce qu'on a en tête, il faut lui expliquer ce
qu'on veut qu'il execute quand on essaie d'accéder à notre app. Pour ça, heroku
utilise un fichier appelé `Procfile` (https://devcenter.heroku.com/articles/procfile).
La doc de heroku explique que le procfile sert à dire quelles commandes lancer 
pour quel type de process. Le seul type de process qui peut capter et rediriger
du traffic web, est un process de type `web` (il en existe d'autres; par ex: 
`release`, `worker`, `urgentworker`)

Bref, si on veut utiliser rocket pour servir le truc, on doit ajouter un fichier
`Procfile` qui va lancer notre server (rocket) et lui permettre de servir du 
traffic web. Lancer notre app est aussi simple que de le faire depuis la command
line: `./target/release/hrku_mytools`. **MAIS** si on veut pouvoir servir du
traffic qui vient de partout dans le monde (a priori c'est ce qu'on veut: il est
très improbable qu'on ait acces au container _aka dynos_) on doit spécifier les 
paramètres suivants: 

* `ROCKET_ADDRESS="0.0.0.0"` pour qu'il puisse servir n'importe quelle connexion
    dans le monde.
* `ROCKET_PORT=$PORT` parce que heroku nous ouvre un port au hasard, qu'on ne 
    connait pas d'avance et qu'on doit pouvoir s'y attacher (sans être root ofc).

Sans ces paramètres, notre appli se déploie bien; mais elle crashe dès qu'on
essaie de faire un truc. In fine, notre `Procfile` minimal ressemble à ça:

```
web: ROCKET_ADDRESS="0.0.0.0" ROCKET_PORT=$PORT ROCKET_KEEP_ALIVE=0 ./target/release/hrku_mytools
```

## Database
Si on veut, on peut utiliser une database (par ex postgres ou redis) et faire 
des trucs funky avec. La documentation montre les différents tarifs etc.. 
(https://elements.heroku.com/addons/heroku-postgresql). Il y a un plan qui est
completement gratuit. Il ne permet pas d'utiliser bcp de resources mais pour
bidouiller, c'est ok. Le plus simple pour ajouter une db postgrest à un 
truc existant c'est de le faire en ligne de commande avec. Ce qui va créer la
db et l'exposer via `$DATABASE_URL` (qu'on devra référencer dans le procfile 
ou utiliser directeement dans le code).

```bash
heroku addons:create heroku-postgresql:hobby-dev 
```

Travailler avec la DB a été étonnament plus compliqué que ce à quoi je 
m'attendais. Pour la lib j'ai simplement opté pour le crate SQLx même si je
sais que le crate `pgsql` est un rien plus efficace. L'idée était d'utiliser un
truc efficace et un peu générique. Un autre avantage de `sqlx` c'est qu'il 
offre un connection pool (et ca, on en a franchement besoin dans la suite).

Le plan de base de heroku pour une DB permet d'avoir 20 connexions en même temps.
C'est beaucoup et c'est peu. Surtout quand on sait quelles ne sont pas libérées
immédiatement. Donc on a besoin d'avoir une "variable globale" qui permette de
stocker le connection pool puis de dispatcher des connexions (qui vont être 
reclaim après utilisation). Apparemment, l'option privilégiée serait d'utiliser
`rocket-contrib` pour faire ca. Ce crate est sensé avoir une feature de gestion
des connexion pools intégrée. Mais la derniere version (celle qui marche avec
rocket 0.5.x ) n'est pas sur crates.io et je n'ai pas réussi à faire en sorte 
de l'utiliser (meme en spécifiant un hash git dans le Cargo.toml). Bref, j'ai 
fini par y aller à la main... et ca marche très bien.

Pour définir une variable globale qui utilise une resource allouée dynamiquement, 
il n'y a pas trop le choix: on doit passer par un appel à `lazy_static`. 
Seulement il y a un problème: `lazy_static` predates `async/await` et il n'y a 
pas moyen de directement charger une variable globale depuis une future. *SAUF* 
que c'est exactement ce que fournit le crate `async_once`. Il permet d'emballer
une future dans un type qui peut être chargé statiquement. Et ne l'appelle que
une seule fois au cours de la vie de l'objet.

Au final, mon code d'initialization du pool ressemble à ça: 
```rust
lazy_static! {
    static ref POOL: AsyncOnce<Result<PgPool, Error>> = AsyncOnce::new(async {
        let uri = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&uri).await?;
        Ok(pool)
    });
}
```

Et l'utilisation de la ressource se fait simplement via un appel à `.get()` qui 
retourne une référence vers le résultat de la future encapsulée dans le 
`async_once` (dans mon cas, il s'agit d'un `Result<PgPool, Error>`). Toutefois,
on ne peut pas obtenir une connexion directement avec 
`POOL.get().await?.acquire().await?` (même si c'est l'idée) car `&Result<T, E>`
n'implémente pas `Try`. C'est pourquoi j'ai défini une petite fonction 
utilitaire qui fait just ça:

```rust
/// Renvoie une connexion vers la DB
pub async fn db() -> Result<PoolConnection<Postgres>, error::Error> {
    let pool = POOL.get().await;
    match pool {
        Err(e) => Err(e.clone()),
        Ok(pool) => {
            let cnx = pool.acquire().await?;
            Ok(cnx)
        }
    }
}
```

Toutes mes interactions avec la DB utilisent une connexion qui leur est fournie
via un appel à `db().await?` et ça marche très bien: on ne consomme plus les 
connexions de façon aussi agressive qu'avant.

## Front End
TODO: je n'ai pas encore essayé quoi que ce soit de ce coté.