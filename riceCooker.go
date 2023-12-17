package main

import (
	"fmt"
	"time"
)

// RiceCooker represents a rice cooker.
type RiceCooker struct {
	cookedRice bool
}

func (rc *RiceCooker) cookRice() string {
	done := make(chan string)

	go func() {
		time.Sleep(4 * time.Second)
		rc.cookedRice = true
		done <- "Rice is cooked!"
	}()

	return <-done
}

func (rc *RiceCooker) keepWarm() string {
	done := make(chan string)

	go func() {
		time.Sleep(3 * time.Second)
		done <- "Food is kept warm!"
	}()

	return <-done
}

func (rc *RiceCooker) steamCook() string {
	done := make(chan string)

	go func() {
		time.Sleep(3 * time.Second)
		done <- "Steaming is done!"
	}()

	return <-done
}

func (rc *RiceCooker) cookSoup() string {
	done := make(chan string)

	go func() {
		time.Sleep(5 * time.Second)
		done <- "Soup is cooked!"
	}()

	return <-done
}

func (rc *RiceCooker) cookDessert() string {
	done := make(chan string)

	go func() {
		time.Sleep(3 * time.Second)
		done <- "Dessert is cooked!"
	}()

	return <-done
}

func (rc *RiceCooker) beep() string {
	return "BEEP BEEP BEEP - Cooking is finished!"
}

func (rc *RiceCooker) start() {
	for {
		fmt.Println("\nWhat would you like to do?")
		fmt.Println("1. Cook rice")
		fmt.Println("2. Keep warm")
		fmt.Println("3. Steam cook")
		fmt.Println("4. Cook soup")
		fmt.Println("5. Cook dessert")
		fmt.Println("6. Quit")

		var choice int
		fmt.Print("Choose an option: ")
		fmt.Scan(&choice)

		switch choice {
		case 1:
			result := rc.cookRice()
			fmt.Println(result)
			fmt.Println(rc.beep())
		case 2:
			result := rc.keepWarm()
			fmt.Println(result)
			fmt.Println(rc.beep())
		case 3:
			result := rc.steamCook()
			fmt.Println(result)
			fmt.Println(rc.beep())
		case 4:
			result := rc.cookSoup()
			fmt.Println(result)
			fmt.Println(rc.beep())
		case 5:
			result := rc.cookDessert()
			fmt.Println(result)
			fmt.Println(rc.beep())
		case 6:
			fmt.Println("Goodbye !")
			return
		default:
			fmt.Println("Invalid choice. Please choose a valid option.")
		}
	}
}

func main() {
	riceCooker := RiceCooker{}
	riceCooker.start()
}
