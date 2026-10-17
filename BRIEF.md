# BRIEF — Mymodule
<!-- Project brief (French). Code, comments, commits and README are in English. -->

Rev. 0 — AAAA-MM-JJ. A valider avant le code.

## 1. Objet

**Mymodule : une phrase — la question que l'outil repond, et a qui.**

Un module de la famille Blackberrylabs : un outil = un verbe, lisible en une phrase. Il
montre, explique, et n'agit jamais a la place de l'utilisateur.

## 2. Pourquoi

- La douleur observee (ou, quand, combien de personnes).
- Ce que les outils existants ne font pas.

## 3. Ce qu'il lit, ce qu'il montre

- Les fichiers / sources lus, par hote ou par emplacement, chacun avec sa source documentaire.
- Les etats ou constats produits, en mots ; les chiffres des cartes (1 a 4).
- Les constats et leur place sur les quatre gravites communes :
  | Constat | Gravite |
  |---|---|
  | … | 1 agir maintenant / 2 a regarder / 3 bon a savoir / 4 information |

## 4. Regles

- **R1 Lecture seule** : rien d'ecrit hors `~/.config/mymodule/` (reglages de l'app).
- **R2 Aucune sortie reseau.**
- **R3 Secrets masques** des la lecture, jamais ecrits.
- **R4 Actions affichees, jamais executees.**
- **R5 Chaque regle sourcee** (documentation, code source ou observation datee).
- **R6 Anglais par defaut, francais en option** ; visage `bbl-ui`.
- **R7 Module de la tour** : `mymodule report` (`bbl-report/1`), `--select` dans l'app.

## 5. Etapes

0. BRIEF valide ; depot local (identite noreply avant le premier commit).
1. Coeur et CLI ; fixtures ; tests (promesses, conformite du rapport) ; passage reel.
2. Maquette validee, puis app.
3. Paquets, depot, release — sur accord. Fiche dans `modules/` de la tour.

## 6. Hors perimetre (v1)

## 7. Decisions

- **M1 AAAA-MM-JJ** — le nom (verifie libre : crates.io, PyPI, npm ; voisins a connaitre).
