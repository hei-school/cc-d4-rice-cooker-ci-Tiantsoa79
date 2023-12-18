import { question } from 'readline-sync';

export class RiceCooker {
  public cookedRice: boolean = false;

  private async cookItem(ingredient: string, successMessage: string): Promise<boolean> {
    try {
      console.log(`Ajout de ${ingredient}...`);
      await this.delay(1000);
      this.beep();
      console.log(`${successMessage} terminée !`);
      return true;
    } catch (error) {
      console.error(`Une erreur s'est produite lors de la cuisson de ${ingredient}: ${error}`);
      return false;
    }
  }
  

  public async cookRice(): Promise<boolean> {
    return this.cookItem("du riz", "Cuisson du riz");
  }

  public async keepWarm(): Promise<boolean> {
    return this.cookItem("de la nourriture", "Maintien au chaud de la nourriture");
  }

  public async steamCook(): Promise<boolean> {
    return this.cookItem("de la nourriture pour la cuisson à la vapeur", "Cuisson à la vapeur");
  }

  public async cookSoup(): Promise<boolean> {
    const waterAdded = await this.cookItem("de l'eau", "Cuisson de la soupe");
    return waterAdded;
  }

  public async cookDessert(): Promise<boolean> {
    return this.cookItem("des ingrédients pour le dessert", "Cuisson du dessert");
  }

  public beep(): void {
    console.log("BEEP BEEP BEEP - Cuisson terminée !");
  }

  public async start(): Promise<void> {
    let choice: number | null = null;

    do {
      console.log("\nQue souhaitez-vous faire ?");
      console.log("1. Cuire du riz");
      console.log("2. Maintenir au chaud");
      console.log("3. Cuisson à la vapeur");
      console.log("4. Cuisson de la soupe");
      console.log("5. Cuisson du dessert");
      console.log("6. Quitter");

      const input = question('Choisissez une option : ');

      choice = Number(input);

      if (isNaN(choice)) {
        console.log("Choix non valide. Veuillez choisir une option valide");
        choice = 0;
      }

      switch (choice) {
        case 1:
          await this.cookRice();
          break;
        case 2:
          await this.keepWarm();
          break;
        case 3:
          await this.steamCook();
          break;
        case 4:
          await this.cookSoup();
          break;
        case 5:
          await this.cookDessert();
          break;
        case 6:
          console.log("Au revoir !");
          break;
      }
    } while (choice !== 6);
  }

  public async delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

if (require.main === module) {
  const riceCooker = new RiceCooker();
  riceCooker.start();
}
