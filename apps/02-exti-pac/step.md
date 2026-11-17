
Les 4 étapes de configuration dans votre code

Pour que l'appui sur le bouton déclenche une interruption, votre programme doit suivre cet ordre précis :
1. Activer les horloges. Activez l'horloge du GPIOA (sur le bus AHB1) et surtout celle du SYSCFG (sur le bus APB2 via RCC_APB2ENR_SYSCFGEN), sinon vos réglages sur le SYSCFG seront ignorés.
2. Configurer PA0 en entrée. Laissez les bits correspondants à PA0 dans GPIOA->MODER à 00 (mode entrée par défaut). Inutile d'activer un pull-down interne, car la résistance externe R39 de la carte s'en charge déjà.
3. Routage dans le SYSCFG. Mettez les 4 bits EXTI0[3:0] du registre SYSCFG->EXTICR[0] à 0000 pour sélectionner le port A (PA0).
4. Détection du front montant dans l'EXTI. Activez le déclenchement sur front montant dans EXTI->RTSR (bit 0 à 1), démasquez l'interruption dans EXTI->IMR (bit 0 à 1), puis activez EXTI0_IRQn dans le NVIC.
