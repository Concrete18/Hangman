import random, os, time, json


def keyboard_interrupt(func):
    """
    Catches all KeyboardInterrupt exceptions.
    Closes with a message and delayed program exit.
    """

    def wrapped(*args, **kwargs):
        try:
            func(*args, **kwargs)
        except KeyboardInterrupt:
            delay = 0.1
            print(f"\nClosing in {delay} second(s)")
            time.sleep(delay)
            exit()

    return wrapped


class Hangman:
    """
    Python Version of the game Hangman.
    """

    # sets up directory for consistency
    script_dir = os.path.dirname(os.path.abspath(__file__))
    os.chdir(script_dir)

    def __init__(self) -> None:
        """
        Sets up the game object.
        """
        self.load_words_list()

    def get_new_word(self):
        """
        Gets a new word for playing
        """
        if self.words_list:
            new_word = random.choice(self.words_list)
            self.words_list.pop(self.words_list.index(new_word))
            self.current_word = new_word
            return self.current_word
        else:
            return False

    def load_words_list(self):
        """
        Loads the words list from a json.
        """
        with open("words_list.json") as file:
            self.words_list = json.load(file)

    def display_stick_man_2(self, parts=0):
        """
        Displays stick man with n parts shown.
        """
        p0 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |                "
            "\n   |                "
            "\n   |                "
            "\n   |                "
            "\n   |________________"
        )
        p1 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |                "
            "\n   |                "
            "\n   |                "
            "\n   |________________"
        )
        p2 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |       |        "
            "\n   |                "
            "\n   |                "
            "\n   |________________"
        )
        p3 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |      /|        "
            "\n   |                "
            "\n   |                "
            "\n   |________________"
        )
        p4 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |      /|\       "
            "\n   |                "
            "\n   |                "
            "\n   |________________"
        )
        p5 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |      /|\       "
            "\n   |      /         "
            "\n   |                "
            "\n   |________________"
        )
        p6 = (
            "\n   |-------|        "
            "\n   |       |        "
            "\n   |       O        "
            "\n   |      /|\       "
            "\n   |      / \       "
            "\n   |                "
            "\n   |________________"
        )
        if parts == 0:
            print(p0)
        elif parts == 1:
            print(p1)
        elif parts == 2:
            print(p2)
        elif parts == 3:
            print(p3)
        elif parts == 4:
            print(p4)
        elif parts == 5:
            print(p5)
        else:
            print(p6)

    def print_hidden_word(self, word, known_letters: list, left_padding=4):
        """
        Prints out the hidden word with only known letters shown.
        """
        final_string_list = []
        missing_count = 0
        for char in word:
            if char.lower() in known_letters:
                final_string_list.append(char)
            else:
                final_string_list.append("_")
                missing_count += 1
        final_string = " ".join(final_string_list)
        print(" " * left_padding, final_string)
        return not missing_count

    def play_again(self):
        """
        Asks if you want to play again.
        """
        response = input("\nDo you want to play again?\n")
        if response.lower() in ["y", "yes"]:
            self.play()
        else:
            print("\nThanks for playing!")
            exit()

    def guess(self):
        """
        Asks for a guess for the current word or letter.
        """
        self.error = None
        guess = input("\nType a letter or a full guess:\n")
        # full guess
        if guess.lower() == self.current_word.lower():
            print("\nYou win!")
            self.play_again()
        # letter guess
        elif len(guess) == 1:
            guess = guess.lower()
            # guessed letter was already chosen
            if guess in self.known_letters:
                self.error = "\nYou already guessed that correctly."
                return
            # guessed letter or word was already used incorrectly
            elif guess in self.incorrect_guess:
                self.error = "\nYou already guessed that incorrectly."
                return
            # guessed letter is in the current word
            elif guess in self.current_word.lower():
                self.known_letters.append(guess)
                return
        # blank response causes a new prompt for a guess again
        elif guess == "":
            self.error = "\nPlease type in a valid guess."
            return
        self.incorrect_guess.append(guess)
        self.losses += 1
        print("\nIncorrect")
        self.display_stick_man_2(self.losses)
        if self.losses == 6:
            print("\nYou lose!")
            self.play_again()

    @keyboard_interrupt
    def play(self):
        """
        Starts playing Hangman
        """
        # setup
        self.current_word = None
        self.known_letters = [" "]
        self.incorrect_guess = []
        self.error = None
        self.losses = 0

        # picks current word
        words_left = self.get_new_word()
        while self.losses < 6 and words_left:
            os.system("cls" if os.name == "nt" else "clear")
            print("Welcome to the game of Hangman\n")
            win = self.print_hidden_word(
                self.current_word,
                self.known_letters,
            )
            self.display_stick_man_2(self.losses)
            if win:
                print("\nYou win!")
                self.play_again()
            # show incorrect guesses
            if self.incorrect_guess:
                wrong_guesses = ", ".join(self.incorrect_guess)
                print(f"\nWrong Guesses: {wrong_guesses}")
            # error
            if self.error:
                print(self.error)
            # guess the word or letter
            self.guess()
        msg = "\nYou win!\nThere are no more words left."
        input(msg)
        return


if __name__ == "__main__":
    App = Hangman()
    App.play()
