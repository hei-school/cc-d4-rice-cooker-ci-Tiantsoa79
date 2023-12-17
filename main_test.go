package main

import (
	"testing"
)

func TestCookRice(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.cookRice()

	if result != "Rice is cooked!" {
		t.Errorf("Expected 'Rice is cooked!', got %s", result)
	}
}

func TestKeepWarm(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.keepWarm()

	if result != "Food is kept warm!" {
		t.Errorf("Expected 'Food is kept warm!', got %s", result)
	}
}

func TestSteamCook(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.steamCook()

	if result != "Steaming is done!" {
		t.Errorf("Expected 'Steaming is done!', got %s", result)
	}
}

func TestCookSoup(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.cookSoup()

	if result != "Soup is cooked!" {
		t.Errorf("Expected 'Soup is cooked!', got %s", result)
	}
}

func TestCookDessert(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.cookDessert()

	if result != "Dessert is cooked!" {
		t.Errorf("Expected 'Dessert is cooked!', got %s", result)
	}
}

func TestBeep(t *testing.T) {
	riceCooker := RiceCooker{}
	result := riceCooker.beep()

	expected := "BEEP BEEP BEEP - Cooking is finished!"
	if result != expected {
		t.Errorf("Expected '%s', got %s", expected, result)
	}
}


