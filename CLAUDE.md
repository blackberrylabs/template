# CLAUDE.md — Mymodule
<!-- Working rules for Claude in this repository (French). Code and docs are in English. -->

Mymodule : une phrase — ce que l'outil repond. Module de la famille Blackberrylabs ; fiche
dans `Projets/00_moi/PROJETS.md`.

## Regles de travail

- BRIEF valide avant le code ; questions a Alexandre une par une, decisions une par une
  avec une recommandation ; PLAN et JOURNAL tenus a chaque etape ; `00_cadrage/` hors git.
- Lecture seule sans exception, aucune sortie reseau, secrets masques et jamais ecrits,
  rien d'ecrit hors `~/.config/mymodule/`. Un etat, pas une note. Actions affichees,
  jamais executees. Chaque regle cite sa source (doc, code source ou `observed`), sa date,
  sa citation : rien de memoire.
- Commits locaux libres ; `git push`, suppression de fichiers, tags, timers : accord
  explicite. Commandes donnees completes, chemin absolu
  (`cd /home/nushoko/Bureau/Logiciels/Mymodule && …`). Avant tout git lance par Alexandre :
  `find .git -name '*.lock' -delete` (le pont laisse des verrous).
- Anglais : code, commentaires, commits, README, interface par defaut. Francais : BRIEF,
  PLAN, JOURNAL, CLAUDE.md, option de langue.
- Seul nom cite : Alexandre Do.
- Un passage reel sur la machine d'Alexandre avant chaque release ; chaque release
  verifiee depuis l'exterieur (paquet telecharge, somme, version, `report` conforme).

## Identite git

Avant le **premier commit** : `git config user.name "Alexandre Do"` et
`git config user.email "277202463+Nimeria2026@users.noreply.github.com"` en config locale ;
verifier par `git log --format='%an <%ae>'` avant tout push.

## Fondations communes — ne pas recopier, completer a la source

- `bbl-hostrules` : fichiers des hotes IA, lecteurs, masquage (si le module lit des hotes).
- `bbl-ui` : palette, briques iced, reglages, i18n. Meme visage que les autres outils.
- `bbl-report` : le contrat de la tour. `mymodule report` produit un `bbl-report/1` ;
  chaque `Kind` a sa place sur les quatre gravites (`model.rs`), ecrite dans le CHANGELOG.

## Reperes

- `crates/core/src/{model,scan,report}.rs`, `tests/promises.rs` (rien d'ecrit, rien de
  reseau, rapport conforme).
- `crates/cli` (`mymodule`, `mymodule report`), `crates/app` (`mymodule-app --select`).
- `docs/new-module.md` de la tour : la marche a suivre complete.
