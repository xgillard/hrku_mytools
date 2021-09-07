# My tools on heroku

To use rust on heroku, use the heroku rust buildpack 
(https://github.com/emk/heroku-buildpack-rust).

Comme expliqué ici: https://dev.to/xinnks/deploy-a-rust-website-on-heroku-1l45
si on veut utiliser rocket pour servir le truc, on doit ajouter un fichier
`Procfile` qui etablit certaines vars d'environnement
`web: ROCKET_ADDRESS="0.0.0.0" ROCKET_PORT=$PORT ROCKET_KEEP_ALIVE=0 ./target/release/hrku_mytools`

Le coup de `ROCKET_ADDRESS="0.0.0.0"` est obligatoire, sinon le server ne bind
pas avec le reste du monde. La variable d'environnement `ROCKET_PORT=$PORT` est
aussi obligatoire. Sans ça: on ne sait pas binder le port que heroku ouvre pour
nous.