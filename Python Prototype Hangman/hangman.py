import random


class Hangman:
    """
    ph
    """

    word_list = ["Wolverine", "Iron Man", "Dead Pool"]
    previous_words = []
    current_word = ""

    def __init__(self) -> None:
        pass

    def get_new_word(self):
        """
        ph
        """
        new_word = random.choice(self.word_list)
        while new_word in self.previous_words:
            new_word = random.choice(self.word_list)
        self.current_word = new_word

    def play(self):
        """
        ph
        """
        print("Welcome to the game of Hangman")
        # picks current word
        self.get_new_word()
        print(self.current_word)


if __name__ == "__main__":
    App = Hangman()
    App.play()
