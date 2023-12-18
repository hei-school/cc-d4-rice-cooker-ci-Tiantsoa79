use std::io;

#[derive(Debug)]
pub struct RiceCooker;   

impl RiceCooker {
    async fn cook_item(&self, ingredient: &str, success_message: &str) -> Result<bool, &'static str> {
        println!("Ajout de {}...", ingredient);
        self.delay(1000).await;
        self.beep();
        println!("{} terminée !", success_message);
        Ok(true)
    }

    pub async fn cook_rice(&self) -> Result<bool, &'static str> {
        self.cook_item("du riz", "Cuisson du riz").await
    }

    pub async fn keep_warm(&self) -> Result<bool, &'static str> {
        self.cook_item("de la nourriture", "Maintien au chaud de la nourriture").await
    }

    pub async fn steam_cook(&self) -> Result<bool, &'static str> {
        self.cook_item("de la nourriture pour la cuisson à la vapeur", "Cuisson à la vapeur").await
    }

    pub async fn cook_soup(&self) -> Result<bool, &'static str> {
        let water_added = self.cook_item("de l'eau", "Cuisson de la soupe").await?;
        Ok(water_added)
    }

    pub async fn cook_dessert(&self) -> Result<bool, &'static str> {
        self.cook_item("des ingrédients pour le dessert", "Cuisson du dessert").await
    }

    fn beep(&self) {
        println!("BEEP BEEP BEEP - Cuisson terminée !");
    }

    async fn start(&self) {
        let mut choice: Option<u32>;

        loop {
            println!("\nQue souhaitez-vous faire ?");
            println!("1. Cuire du riz");
            println!("2. Maintenir au chaud");
            println!("3. Cuisson à la vapeur");
            println!("4. Cuisson de la soupe");
            println!("5. Cuisson du dessert");
            println!("6. Quitter");
            println!("Choisissew une option : ");
          
            let input = question("Choisissez une option : ");

            choice = match input.trim().parse() {
                Ok(num) => Some(num),
                Err(_) => {
                    println!("Choix non valide. Veuillez choisir une option valide");
                    None
                }
            };

            match choice {
                Some(1) => {
                    if let Err(err) = self.cook_rice().await {
                        eprintln!("{}", err);
                    }
                }
                Some(2) => {
                    if let Err(err) = self.keep_warm().await {
                        eprintln!("{}", err);
                    }
                }
                Some(3) => {
                    if let Err(err) = self.steam_cook().await {
                        eprintln!("{}", err);
                    }
                }
                Some(4) => {
                    if let Err(err) = self.cook_soup().await {
                        eprintln!("{}", err);
                    }
                }
                Some(5) => {
                    if let Err(err) = self.cook_dessert().await {
                        eprintln!("{}", err);
                    }
                }
                Some(6) => {
                    println!("Au revoir !");
                    break;
                }
                _ => {
                    println!("Choix non valide. Veuillez choisir une option valide");
                }
            }
        }
    }

    async fn delay(&self, ms: u64) {
        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
    }
}

fn question(prompt: &str) -> String {
    print!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[tokio::main]
async fn main() {
    let rice_cooker = RiceCooker;
    rice_cooker.start().await;
}



#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_cook_rice() {
      let rice_cooker = RiceCooker;
      let result = rice_cooker.cook_rice().await;
      assert_eq!(result, Ok(true));
  }

  #[tokio::test]
  async fn test_keep_warm() {
      let rice_cooker = RiceCooker;
      let result = rice_cooker.keep_warm().await;
      assert_eq!(result, Ok(true));
  }

  #[tokio::test]
  async fn test_steam_cook() {
      let rice_cooker = RiceCooker;
      let result = rice_cooker.steam_cook().await;
      assert_eq!(result, Ok(true));
  }

  #[tokio::test]
  async fn test_cook_soup() {
      let rice_cooker = RiceCooker;
      let result = rice_cooker.cook_soup().await;
      assert_eq!(result, Ok(true));
  }

  #[tokio::test]
  async fn test_cook_dessert() {
      let rice_cooker = RiceCooker;
      let result = rice_cooker.cook_dessert().await;
      assert_eq!(result, Ok(true));
  }
}