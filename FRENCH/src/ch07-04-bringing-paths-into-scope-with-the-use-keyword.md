<!--
## Bringing Paths into Scope with the `use` Keyword
-->

## Importer des chemins dans la portée via le mot-clé `use`

<!--
It might seem like the paths we’ve written to call functions so far are
inconveniently long and repetitive. For example, in Listing 7-7, whether we
chose the absolute or relative path to the `add_to_waitlist` function, every
time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and
`hosting` too. Fortunately, there’s a way to simplify this process. We can
bring a path into a scope once and then call the items in that path as if
they’re local items with the `use` keyword.
-->

Les chemins que nous avons créés peuvent désormais paraître génants car trop
longs et répétitifs. Par exemple, dans l'encart 7-7, si nous avions choisi
d'utiliser le chemin absolu ou relatif pour la fonction
`ajouter_a_la_liste_attente`, nous devrions aussi écrire `salle_a_manger` et
`accueil` à chaque fois que nous voullions appeler `ajouter_a_la_liste_attente`.
Heureusement, il existe une solution pour simplifier ce cheminement.
Nous pouvons importer un chemin dans la portée et appeler ensuite les éléments
de ce chemin comme s'ils étaient des locaux grâce au mot-clé `use`.

<!--
In Listing 7-11, we bring the `crate::front_of_house::hosting` module into the
scope of the `eat_at_restaurant` function so we only have to specify
`hosting::add_to_waitlist` to call the `add_to_waitlist` function in
`eat_at_restaurant`.
-->

Dans l'encart 7-11, nous importons le module `crate::salle_a_manger::accueil`
dans la portée de la fonction `manger_au_restaurant` afin que nous n'ayons plus
qu'à utiliser `accueil::ajouter_a_la_liste_attente` pour appeler la fonction
`ajouter_a_la_liste_attente` dans `manger_au_restaurant`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```
-->

```rust
mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

use crate::salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
}
# fn main() {}
```

<!--
<span class="caption">Listing 7-11: Bringing a module into scope with
`use`</span>
-->

<span class="caption">Encart 7-11 : importer un module dans la portée via `use`
</span>

<!--
Adding `use` and a path in a scope is similar to creating a symbolic link in
the filesystem. By adding `use crate::front_of_house::hosting` in the crate
root, `hosting` is now a valid name in that scope, just as though the `hosting`
module had been defined in the crate root. Paths brought into scope with `use`
also check privacy, like any other paths.
-->

Dans une portée, utiliser un `use` et un chemin revient à créer un lien
symbolique dans le système de fichier. Grâce à l'ajout de
`use crate::salle_a_manger::accueil` dans la crate racine, `accueil` est
maintenant un nom valide dans cette portée, comme si le module `accueil` avait
été défini dans la crate racine. Les chemins importés dans la portée via `use`
sont soumis au principe de protection, tout comme les autres chemins.

<!--
You can also bring an item into scope with `use` and a relative path. Listing
7-12 shows how to specify a relative path to get the same behavior as in
Listing 7-11.
-->

Vous pouvez aussi importer un élément dans portée avec `use` et un chemin
relatif. L'encart 7-12 nous montre comment utiliser un
chemin relatif pour obtenir le même résultat que l'encart 7-11.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```
-->

```rust
mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

use salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
}
# fn main() {}
```

<!--
<span class="caption">Listing 7-12: Bringing a module into scope with `use` and
a relative path</span>
-->

<span class="caption">Encart 7-12 : importer un module dans la portée avec `use`
et un chemin relatif</span>

<!--
### Creating Idiomatic `use` Paths
-->

### Créer des chemins idéaux pour `use`

<!--
In Listing 7-11, you might have wondered why we specified `use
crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in
`eat_at_restaurant` rather than specifying the `use` path all the way out to
the `add_to_waitlist` function to achieve the same result, as in Listing 7-13.
-->

Dans l'encart 7-11, vous vous êtes peut-être demandé pourquoi nous avions
utilisé `use crate::salle_a_manger::accueil` et appelé ensuite
`accueil::ajouter_a_la_liste_attente` dans `manger_au_restaurant` plutôt que
d'écrire le chemin du `use` jusqu'à la fonction `ajouter_a_la_liste_attente`
pour avoir le même résultat, comme dans l'encart 7-13.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
# fn main() {}
```
-->

```rust
mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

use crate::salle_a_manger::accueil::ajouter_a_la_liste_attente;

pub fn manger_au_restaurant() {
    ajouter_a_la_liste_attente();
    ajouter_a_la_liste_attente();
    ajouter_a_la_liste_attente();
}
# fn main() {}
```

<!--
<span class="caption">Listing 7-13: Bringing the `add_to_waitlist` function
into scope with `use`, which is unidiomatic</span>
-->

<span class="caption">Encart 7-13 : importer la fonction
`ajouter_a_la_liste_attente` dans la portée avec `use`, ce qui n'est pas idéal
</span>

<!--
Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is
the idiomatic way to bring a function into scope with `use`. Bringing the
function’s parent module into scope with `use` so we have to specify the parent
module when calling the function makes it clear that the function isn’t locally
defined while still minimizing repetition of the full path. The code in Listing
7-13 is unclear as to where `add_to_waitlist` is defined.
-->

Bien que l'encart 7-11 et 7-13 accomplissent la même tâche, l'encart 7-11 est la
façon idéale d'importer une fonction dans la portée via `use`. Le fait
d'importer le module parent de la fonction dans notre portée avec `use`, de
sorte que nous ayons à préciser le module parent quand nous appelons la fonction
précise clairement que la fonction n'est pas définie localement, tout en
minimisant la répétition du chemin complet. Nous ne pouvons pas en déduire
facilement où est défini `ajouter_a_la_liste_attente` dans l'encart 7-13.

<!--
On the other hand, when bringing in structs, enums, and other items with `use`,
it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way
to bring the standard library’s `HashMap` struct into the scope of a binary
crate.
-->

Cela dit, lorsque nous importons des structures, des énumérations, et d'autres
éléments avec `use`, il est idéal de préciser le chemin complet. L'encart 7-14
montre la manière idéale d'importer la structure `HashMap` de la bibliothèque
standard dans la portée d'une crate binaire.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

<!--
<span class="caption">Listing 7-14: Bringing `HashMap` into scope in an
idiomatic way</span>
-->

<span class="caption">Encart 7-14 : import de `HashMap` dans la portée de
manière idéale</span>

<!--
There’s no strong reason behind this idiom: it’s just the convention that has
emerged, and folks have gotten used to reading and writing Rust code this way.
-->

Il n'y a pas de forte justification à cette pratique : c'est simplement une
convention qui a germé, et les gens se sont habitués à lire et écrire du code
Rust de cette façon.

<!--
The exception to this idiom is if we’re bringing two items with the same name
into scope with `use` statements, because Rust doesn’t allow that. Listing 7-15
shows how to bring two `Result` types into scope that have the same name but
different parent modules and how to refer to them.
-->

Il y a une exception à cette pratique : nous ne pouvons pas utiliser
l'instruction `use` pour importer deux éléments avec le même nom dans la portée,
car Rust ne l'autorise pas. L'encart 7-15 nous montre comment importer puis
utiliser deux types `Result` ayant le même nom mais dont leurs modules parents
sont distincts.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
#     Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
#     Ok(())
}
```
-->

```rust
use std::fmt;
use std::io;

fn fonction1() -> fmt::Result {
    // -- code masqué ici --
#     Ok(())
}

fn fonction2() -> io::Result<()> {
    // -- code masqué ici --
#     Ok(())
}
```

<!--
<span class="caption">Listing 7-15: Bringing two types with the same name into
the same scope requires using their parent modules.</span>
-->

<span class="caption">Encart 7-15 : l'import de deux types ayant le même nom
dans la même portée nécessite d'utiliser leurs modules parents.</span>

<!--
As you can see, using the parent modules distinguishes the two `Result` types.
If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d
have two `Result` types in the same scope and Rust wouldn’t know which one we
meant when we used `Result`.
-->

Comme vous pouvez le constater, l'utilisation des modules parents permet de
distinguer les deux types `Result`. Si nous avions utilisé
`use std::fmt::Result` et `use std::io::Result`, nous aurions deux types de
`Result` dans la même portée et donc Rust ne pourrait pas comprendre lequel nous
utiliserions en demandant `Result`.

<!--
### Providing New Names with the `as` Keyword
-->

### Renommer des éléments avec le mot-clé `as`

<!--
There’s another solution to the problem of bringing two types of the same name
into the same scope with `use`: after the path, we can specify `as` and a new
local name, or alias, for the type. Listing 7-16 shows another way to write the
code in Listing 7-15 by renaming one of the two `Result` types using `as`.
-->

Il y a une autre solution au fait d'avoir deux types du même nom dans la même
portée à cause de `use` : après le chemin, nous pouvons rajouter `as` suivi d'un
nouveau nom local, ou alias, sur le type. L'encart 7-16 nous montre une autre
façon d'écrire le code de l'encart 7-15 en utilisant `as` pour renommant un des
deux types `Result`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
#     Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
#     Ok(())
}
```
-->

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn fonction1() -> Result {
    // -- code masqué ici --
#     Ok(())
}

fn fonction2() -> IoResult<()> {
    // -- code masqué ici --
#     Ok(())
}
```

<!--
<span class="caption">Listing 7-16: Renaming a type when it’s brought into
scope with the `as` keyword</span>
-->

<span class="caption">Encart 7-16 : renommer un type lorsqu'il est importé dans
la portée, avec le mot-clé `as`</span>

<!--
In the second `use` statement, we chose the new name `IoResult` for the
`std::io::Result` type, which won’t conflict with the `Result` from `std::fmt`
that we’ve also brought into scope. Listing 7-15 and Listing 7-16 are
considered idiomatic, so the choice is up to you!
-->

Dans la seconde instruction `use`, nous avons choisi `IoResult` comme nouveau
nom du type `std::io::Result`, qui n'est plus en conflit avec le `Result` de
`std::fmt` que nous avons aussi importé dans la portée. Les encarts 7-15 et 7-16
sont idéaux, donc le choix vous revient ! 

<!--
### Re-exporting Names with `pub use`
-->

### Re-exporter des éléments avec `pub use`

<!--
When we bring a name into scope with the `use` keyword, the name available in
the new scope is private. To enable the code that calls our code to refer to
that name as if it had been defined in that code’s scope, we can combine `pub`
and `use`. This technique is called *re-exporting* because we’re bringing
an item into scope but also making that item available for others to bring into
their scope.
-->

Lorsque nous importons un élément dans la portée avec le mot-clé `use`, son nom
dans la nouvelle portée est privé. Pour permettre au code appelant d'utiliser ce
nom comme s'il était défini dans cette portée, nous pouvons associer `pub` et
`use`. Cette technique est appelée *re-exporter* car nous importons un
élément dans la portée, mais nous rendons aussi cet élément disponible aux
portées des autres.

<!--
Listing 7-17 shows the code in Listing 7-11 with `use` in the root module
changed to `pub use`.
-->

L'encart 7-17 nous montre le code de l'encart 7-11 où le `use` du module racine
a été remplacé par `pub use`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
# fn main() {}
```
-->

```rust
mod salle_a_manger {
    pub mod accueil {
        pub fn ajouter_a_la_liste_attente() {}
    }
}

pub use crate::salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
}
# fn main() {}
```

<!--
<span class="caption">Listing 7-17: Making a name available for any code to use
from a new scope with `pub use`</span>
-->

<span class="caption">Encart 7-17 : rendre un élément disponible pour n'importe
quel code qui l'importera dans sa portée, avec `pub use`</span>

<!--
By using `pub use`, external code can now call the `add_to_waitlist` function
using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the
`eat_at_restaurant` function could call `hosting::add_to_waitlist` in its
scope, but external code couldn’t take advantage of this new path.
-->

Grâce à `pub use`, le code externe peut maintenant appeler la fonction
`ajouter_a_la_liste_attente` en utilisant `accueil::ajouter_a_la_liste_attente`.
Si nous n'avions pas utilisé `pub use`, la fonction `manger_au_restaurant`
aurait pu appeler `accueil::ajouter_a_la_liste_attente` dans sa portée, mais le
code externe n'aurait pas pu profiter de ce nouveau chemin.

<!--
Re-exporting is useful when the internal structure of your code is different
from how programmers calling your code would think about the domain. For
example, in this restaurant metaphor, the people running the restaurant think
about “front of house” and “back of house.” But customers visiting a restaurant
probably won’t think about the parts of the restaurant in those terms. With
`pub use`, we can write our code with one structure but expose a different
structure. Doing so makes our library well organized for programmers working on
the library and programmers calling the library.
-->

Re-exporter est utile quand la structure interne de votre code est différente de
comment les développeurs qui utilisent votre code imaginent le domaine. Par
exemple, dans cette métaphore du restaurant, les personnes qui font fonctionner
le restaurant se structurent en fonction de la “salle à manger” et des
“cuisines”. Mais les clients qui utilisent le restaurant ne vont probablement
voir les choses ainsi. Avec `pub use`, nous pouvons écrire notre code selon une
certaine organisation, mais l'exposer avec une organisation différente. En
faisant ainsi, la bibliothèque est bien organisée autant pour les développeurs
qui travaillent dans la bibliothèque que les développeurs qui utilisent la
bibliothèque.

<!--
### Using External Packages
-->

### Utiliser des paquets externes

<!--
In Chapter 2, we programmed a guessing game project that used an external
package called `rand` to get random numbers. To use `rand` in our project, we
added this line to *Cargo.toml*:
-->

Dans le chapitre 2, nous avions développé un projet de jeu de devinettes qui
utilisait le paquet externe `rand` afin d'obtenir des nombres aléatoires.
Pour pouvoir utiliser `rand` dans notre projet, nous avons ajouté cette ligne
dans *Cargo.toml* :

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

```toml
[dependencies]
rand = "0.5.5"
```

<!--
Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the
`rand` package and any dependencies from [crates.io](https://crates.io/) and
make `rand` available to our project.
-->

L'ajout de `rand` comme dépendance dans *Cargo.toml* demande à Cargo de
télécharger le paquet `rand` et toutes ses dépendances à partir de
[crates.io](https://crates.io/) et rend disponible `rand` pour notre projet.

<!--
Then, to bring `rand` definitions into the scope of our package, we added a
`use` line starting with the name of the package, `rand`, and listed the items
we wanted to bring into scope. Recall that in the [“Generating a Random
Number”][rand]<!-- ignore -- > section in Chapter 2, we brought the `Rng` trait
into scope and called the `rand::thread_rng` function:
-->

Ensuite, pour importer les définitions de `rand` dans la portée de notre paquet,
nous avons ajouté une ligne `use` qui commence avec le nom de notre paquet,
`rand`, et nous avons listé les éléments que nous voulions importer dans notre
portée. Dans la section [“Générer le nombre secret”][rand]<!-- ignore --> du
chapitre 2, nous avons importé le trait `Rng` dans la portée, puis nous avons
appelé la fonction `rand::thread_rng` :

<!--
```rust,ignore
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```
-->

```rust,ignore
use rand::Rng;
fn main() {
    let nombre_secret = rand::thread_rng().gen_range(1, 101);
}
```

<!--
Members of the Rust community have made many packages available at
[crates.io](https://crates.io/), and pulling any of them into your package
involves these same steps: listing them in your package’s *Cargo.toml* file and
using `use` to bring items into scope.
-->

Les membres de la communauté Rust ont mis à disposition de nombreux paquets
dans [crates.io](https://crates.io/), et utiliser l'un d'entre eux dans votre
paquet implique toujours ces mêmes étapes : les lister dans le fichier
*Cargo.toml* de votre paquet et utiliser `use` pour les importer dans la portée.

<!--
Note that the standard library (`std`) is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`. But we do need to refer to
it with `use` to bring items from there into our package’s scope. For example,
with `HashMap` we would use this line:
-->

Notez que la bibliothèque standard (`std`) est aussi une crate qui est externe à
notre paquet. Comme la bibliothèque standard est livrée avec le langage Rust,
nous n'avons pas à modifier le *Cargo.toml* pour y inclure `std`. Mais nous
devons utiliser `use` pour importer les éléments qu'y se trouvent dans la portée
de notre paquet. Par exemple, pour `HashMap` nous pourrions utiliser cette
ligne :

```rust
use std::collections::HashMap;
```

<!--
This is an absolute path starting with `std`, the name of the standard library
crate.
-->

C'est un chemin absolu qui commence par `std`, le nom de la crate de la
bibliothèque standard.

<!--
### Using Nested Paths to Clean Up Large `use` Lists
-->

### Utiliser des chemins imbriqués pour simplifier les grandes listes de `use`

<!--
If we’re using multiple items defined in the same package or same module,
listing each item on its own line can take up a lot of vertical space in our
files. For example, these two `use` statements we had in the Guessing Game in
Listing 2-4 bring items from `std` into scope:
-->

Si vous utilisez de nombreux éléments définis dans un même paquet ou dans un
même module, lister chaque élément sur sa propre ligne prendra beaucoup d'espace
vertical dans vos fichiers. Par exemple, ces deux instructions `use`, que nous
avions dans le jeu de devinettes, dans l'encart 2-4, importaient des éléments de
`std` dans la portée :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
use std::io;
use std::cmp::Ordering;
// ---snip---
```
-->

```rust
use std::io;
use std::cmp::Ordering;
// --- code masqué ici ---
```

<!--
Instead, we can use nested paths to bring the same items into scope in one
line. We do this by specifying the common part of the path, followed by two
colons, and then curly brackets around a list of the parts of the paths that
differ, as shown in Listing 7-18.
-->

A la place, nous pouvons utiliser des chemins imbriqués afin d'importer les
mêmes éléments dans la portée en une seule ligne. Nous pouvons faire cela en
indiquant la partie commune du chemin, suivi de deux double-points, puis
d'accolades autour d'une liste d'éléments du chemin, comme dans l'encart 7-18 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
use std::{cmp::Ordering, io};
// ---snip---
```
-->

```rust
use std::{cmp::Ordering, io};
// --- code masqué ici ---
```

<!--
<span class="caption">Listing 7-18: Specifying a nested path to bring multiple
items with the same prefix into scope</span>
-->

<span class="caption">Encart 7-18 : utiliser un chemin imbriqué pour importer
plusieurs éléments avec le même préfixe dans la portée</span>

<!--
In bigger programs, bringing many items into scope from the same package or
module using nested paths can reduce the number of separate `use` statements
needed by a lot!
-->

Pour des programmes plus gros, importer plusieurs éléments dans la portée pour
le même paquet ou module en utilisant des chemins imbriqués peut réduire
considérablement le nombre de `use` utilisés !

<!--
We can use a nested path at any level in a path, which is useful when combining
two `use` statements that share a subpath. For example, Listing 7-19 shows two
`use` statements: one that brings `std::io` into scope and one that brings
`std::io::Write` into scope.
-->

Nous pouvons utiliser un chemin imbriqué à tous les niveaux d'un chemin, ce qui
peut être utile lorsqu'on utilise deux instructions `use` qui partagent un
sous-chemin. Par exemple, l'encart 7-19 nous montre deux instructions `use` :
un qui importe `std::io` dans la portée et un autre qui importe `std::io::Write`
dans la portée.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust
use std::io;
use std::io::Write;
```

<!--
<span class="caption">Listing 7-19: Two `use` statements where one is a subpath
of the other</span>
-->

<span class="caption">Encart 7-19 : deux instructions `use` où l'une est un
sous-chemin de l'autre</span>

<!--
The common part of these two paths is `std::io`, and that’s the complete first
path. To merge these two paths into one `use` statement, we can use `self` in
the nested path, as shown in Listing 7-20.
-->

La partie commune entre ces deux chemins est `std::io`, et c'est le premier
chemin complet. Pour imbriquer ces deux chemins en une seule instruction `use`,
nous pouvons utiliser `self` dans le chemin imbriqué, comme dans l'encart 7-20.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

```rust
use std::io::{self, Write};
```

<!--
<span class="caption">Listing 7-20: Combining the paths in Listing 7-19 into
one `use` statement</span>
-->

<span class="caption">Encart 7-20 : imbrication des chemins de l'encart 7-19
dans une seule instruction `use`</span>

<!--
This line brings `std::io` and `std::io::Write` into scope.
-->

Cette ligne importe `std::io` et `std::io::Write` dans la portée.

<!--
### The Glob Operator
-->

### L'opérateur global

<!--
If we want to bring *all* public items defined in a path into scope, we can
specify that path followed by `*`, the glob operator:
-->

Si nous voulons importer, dans la portée, *tous* les éléments publics définis
dans un chemin, nous pouvons indiquer ce chemin suivi par `*`, l'opérateur
global :

<!--
```rust
use std::collections::*;
```
-->

```rust
use std::collections::*;
```

<!--
This `use` statement brings all public items defined in `std::collections` into
the current scope. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program
was defined.
-->

Cette instruction `use` va importer tous les éléments publics définis dans
`std::collections` dans la portée courante. Mais soyez prudent quand vous
utilisez l'opérateur global ! L'opérateur global rend difficile à dire quels
éléments sont dans la portée et où un élément est utilisé dans notre programme.

<!--
The glob operator is often used when testing to bring everything under test
into the `tests` module; we’ll talk about that in the [“How to Write
Tests”][writing-tests]<!-- ignore -- > section in Chapter 11. The glob operator
is also sometimes used as part of the prelude pattern: see [the standard
library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -- >
for more information on that pattern.
-->

L'opérateur global est souvent utilisé lorsque nous écrivons des tests, pour
importer tout ce qui a à tester dans le module `tests` ; nous verrons cela dans
une section du [chapitre 11][writing-tests]. L'opérateur global est parfois
aussi utilisé pour l'étape préliminaire : rendez-vous dans [la documentation de
la bibliothèque
standard](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)<!--
ignore --> pour plus d'informations sur cela.

<!--
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
-->

[rand]: ch02-00-guessing-game-tutorial.html#générer-le-nombre-secret
[writing-tests]: ch11-01-writing-tests.html