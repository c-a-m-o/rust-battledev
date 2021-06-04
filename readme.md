# Rust battledev

La compétition d'algorithmique [BattleDev](https://battledev.blogdumoderateur.com/) supporte les langages suivants :
- C
- C#
- C++
- Go
- Java
- Kotlin
- NodeJS
- PHP
- Python
- Ruby
- Scala
- Swift

Cependant, il n'est pas possible de participer dans le langage Rust. Plutôt que de chercher à se plaindre qu'un langage aussi merveilleux ne soit pas supporté, l'objectif de ce projet est de montrer qu'il est en fait possible de participer au concours en Rust.

Le concept consiste d'abord à compiler le code source Rust en local vers une cible WebAssembly. WebAssembly étant supporté par NodeJS, il est possible d'exécuter le code compilé lors d'une soumission de code NodeJS. Afin d'éviter tout problème de copié / collé et d'envoi du code, le wasm compilé est encodé en base64 puis décodé à l'exécution.

Comme je ne connais pas encore les subtilités de WebAsssembly, le template de code provient en majorité du code généré par l'outil `wasm-pack`.

### Utilisation

Tout d'abord il faut avoir installé Rust et NodeJS.
Ensuite, si `wasm-pack` n'est pas installé sur votre machine :
```sh
cargo install wasm-pack
```

Pour générer le code source NodeJS pour l'exercice N :
- Décommenter `mod exN;` et `exN::main()` dans `lib.rs`.
- Lancer le script build via
```sh
node ./build.js
``` 
Le code sera généré dans un fichier `output.js`.

Si jamais à l'exécution un message d'erreur de ce type
```
LinkError: WebAssembly.Instance(): Import #0 module="__wbindgen_placeholder__" function="__wbg_print_b630494e95fbe301" error: function import requires a callable
```
apparaît, c'est qu'il faut changer le nom de la fonction print (ou readline selon le cas) à la ligne 131 (ou 124) du template par le nom spécifié.

Les codes générés ont été testés sur [la plateforme Isograd](https://www.isograd.com/FR/solutionconcours.php) et correspondent aux solutions des exercices de la BattleDev de novembre 2020.

