# STM32F407 Rust Embedded Lab

Laboratoire expérimental de développement embarqué en **Rust** sur **STM32F407**, consacré à la mise en place d'une chaîne **CI/CD** combinant compilation croisée, simulation avec **Renode** et validation sur **carte physique**.

L'objectif principal est d'étudier dans quelle mesure Renode peut être intégré à une chaîne d'intégration continue pour automatiser les tests de firmwares embarqués, d'identifier ses limites et de déterminer quels tests nécessitent une validation sur matériel réel.

## Objectifs

- Développer des firmwares Rust `no_std` pour STM32F407, du **PAC** au **HAL** (`stm32f4xx-hal`).
- Construire progressivement une chaîne **CI/CD avec GitHub Actions**.
- Automatiser la compilation, les tests et les validations en simulation avec **Renode**.
- Confronter les résultats obtenus sous Renode à ceux observés sur la **STM32F407G-DISC1**.
- Identifier les périphériques, comportements et scénarios difficilement ou non testables en simulation.
- Explorer l'intégration de tests **HIL** (*Hardware-in-the-Loop*) dans une chaîne CI/CD.
- Documenter les écarts entre simulation et matériel réel afin d'évaluer la fiabilité et les limites de cette approche.