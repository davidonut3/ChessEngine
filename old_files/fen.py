from config import *
from visual.piece import Piece

class Fenn:
    DEFAULT = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1'
    RANDOM = 'r1bq1rk1/pppp1ppp/2n2n2/2b1p3/4P3/2NP1N2/PPP2PPP/R1BQ1RK1 b - e3 0 7'
    CASTLE = 'r3k2r/ppp2ppp/8/3Pp3/8/8/PPP2PPP/R3K2R w KQkq e6 0 1'
    CHECK = 'b3k3/8/8/8/4r3/8/4N3/R3K2R w KQkq - 0 1'

    def __init__(self, fen=DEFAULT):
        self.parse(fen)
        self.moves = []

    def __str__(self):
        return self.board + ' ' + self.turn + ' ' + self.castle + ' ' + self.enpassant + ' ' + self.halfmove + ' ' + self.fullmove
    
    def list(self):
        return [self.board, self.turn, self.castle, self.enpassant, self.halfmove, self.fullmove]
    
    def parse(self, fen):
        info = fen.split()
        self.board = info[0]
        self.turn = info[1]
        self.castle = info[2]
        self.enpassant = info[3]
        self.halfmove = info[4]
        self.fullmove = info[5]

    def fen_to_visual(self, board, perspective):
        ranks = self.board.split('/')

        for i in range(8):
            rank = ranks[i]
            index = 0
            for string in rank:
                if string.isdigit():
                    num = int(string)
                    index += num - 1
                    tile = board[i][index]
                    tile.has_piece = False
                    tile.piece = NO_PIECE
                else:
                    piece_name = letter_to_name[string]
                    piece_type = letter_to_type[string]
                    piece_bounds = letter_to_bounds[string]
                    tile = board[i][index]
                    tile.set_piece(Piece(piece_name, piece_type, perspective, piece_bounds))
                index += 1
        
        return board
    
    def is_legal_move(self, start, end):
        return is_legal_move(start, end, self)

    def move(self, start, end):
        if not start and not end:
            return False

        if is_legal_move(start, end, self):
            move = move_to_lan(start, end)
            self.moves.append(move)
            self.parse(lan_to_fen(self, move))

            return True
        
        return False
    
    def game_ended(self):
        game_has_ended = game_ended(self)
        if game_has_ended == "0-1" or game_has_ended == "1-0" or game_has_ended == "½-½":
            self.moves.append(game_has_ended)
            return True
        return False
        
    def get_possible_moves(self, tile):
        return get_possible_moves(self, [tile.pos[1], tile.pos[0]])
    
    def get_all_possible_moves(self):
        return get_all_possible_moves(self)