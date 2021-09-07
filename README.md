# My tools on heroku

To use rust on heroku, use the heroku rust buildpack 
(https://github.com/emk/heroku-buildpack-rust).

Comme expliqué ici: https://dev.to/xinnks/deploy-a-rust-website-on-heroku-1l45
si on veut utiliser rocket pour servir le truc, on doit ajouter un fichier
`Procfile` qui etablit certaines vars d'environnement
`web: ROCKET_PORT=$PORT ROCKET_KEEP_ALIVE=0 ./target/release/hrku_mytools`