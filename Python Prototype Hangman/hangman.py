import random


class Hangman:
    """
    ph
    """

    word_list = ["Wolverine", "Iron Man", "Dead Pool"]
    current_word = ""

    def __init__(self) -> None:
        """
        ph
        """
        pass

    def get_new_word(self):
        """
        ph
        """
        if self.word_list:
            new_word = random.choice(self.word_list)
            self.word_list.pop(self.word_list.index(new_word))
            self.current_word = new_word
            return self.current_word
        else:
            return False

    def display_stick_man(self, removed_limbs=0, left_padding=4):
        """
        ph
        """
        stick_man = [
            ["|", "-", "-", "-", "-", "|", " ", " ", " ", " "],
            ["|", " ", " ", " ", " ", "|", " ", " ", " ", " "],
            ["|", " ", " ", " ", " ", "O", " ", " ", " ", " "],
            ["|", " ", " ", " ", "/", "|", "\\", " ", " ", " "],
            ["|", " ", " ", " ", "/", " ", "\\", " ", " ", " "],
            ["|", "_", "_", "_", "_", "_", "_", "_", "_", "_"],
        ]
        remove_order = [
            (4, 6),  # right leg
            (4, 4),  # left leg
            (3, 6),  # right arn
            (3, 4),  # left arm
            (3, 5),  # body
            (2, 5),  # head
        ]
        for num in range(removed_limbs):
            x, y = remove_order[num]
            stick_man[x][y] = " "
        for line in stick_man:
            padding = " " * left_padding
            print(f'{padding}{"".join(line)}')

    def print_hidden_word(self, word, known_chars: list = []):
        """
        ph
        """
        final_string_list = []
        known_chars.append(" ")
        for char in word:
            if char.lower() in known_chars:
                final_string_list.append(char)
            else:
                final_string_list.append("_")
        final_string = " ".join(final_string_list)
        print(final_string)

    def play(self):
        """
        ph
        """
        print("Welcome to the game of Hangman")
        # picks current word
        if not self.get_new_word():
            msg = "You win! There are no more words left."
            print(msg)
            return
        self.print_hidden_word(self.current_word)


if __name__ == "__main__":
    App = Hangman()
    App.play()
    # App.print_hidden_word("Iron Man", ["t", "a", "e", "i"])
    # App.display_stick_man(4)
