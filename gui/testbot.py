from config import *

class TestBot:
    def class_name():
        return 'testbot'

    def __init__(self, fen: FenPy, delay=0.1):
        self.fen = fen
        self.delay = delay

    def get_move(self):
        time.sleep(self.delay)
        moves = self.fen.get_all_possible_moves_lan()
        index = random.randint(0, len(moves) - 1)
        move = moves[index]
        # print('----')
        # print(self.fen.to_string())
        # print(moves)
        # print(move)
        self.fen.lan_to_fen(move)
        # print(self.fen.to_string())
        return move
    
    def receive_move(self, move):
        self.fen.lan_to_fen(move)