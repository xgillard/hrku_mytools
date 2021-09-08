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
db et l'exposer via `$DATABASE_URL` (qu'on devra référencer dans le procfile).

```bash
heroku addons:create heroku-postgresql:hobby-dev 
```