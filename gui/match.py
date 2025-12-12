from config import *
from visual.utils import *
from visual.visual import Visual


class PlayerVsPlayerMatch:
    def __init__(self, fen=FenPy(), perspective=WHITE):
        self.fen = fen
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
        print("Player vs player match")

        self.visual.setup()
            
        running = True
        while running:
            if self.fen.white_to_move():
                move = self.visual.get_move(WHITE)
            else:
                move = self.visual.get_move(BLACK)

            if self.visual.fen != self.fen:
                self.fen = self.visual.fen
                continue
            
            if not move:
                running = False
            else:
                self.fen.lan_to_fen(move)
                self.visual.update_fen_list()

            game_ended = self.fen.game_ended()
            if game_ended == WHITE_WINS or game_ended == BLACK_WINS or game_ended == DRAW:
                print(game_ended)
                running = False

        pygame.quit()


class BotVsBotMatch:
    def __init__(self, white, black, delay_seconds=0, fen=FenPy(), is_visual=True, perspective=WHITE):
        self.white = white
        self.black = black
        self.delay_seconds = delay_seconds
        self.fen = fen
        self.is_visual = is_visual or self.white == USER or self.black == USER
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
        print("Bot vs bot match")

        win = NOT_ENDED

        if self.is_visual:
            self.visual.setup()

        running = True
        while running:
            if self.is_visual:
                if not self.visual.run_static(self):
                    running = False
            else:
                running = self.get_move()

            game_ended = self.fen.game_ended()
            if game_ended == WHITE_WINS or game_ended == BLACK_WINS or game_ended == DRAW:

                winner = None
                if game_ended == WHITE_WINS:
                    winner = self.white
                if game_ended == BLACK_WINS:
                    winner = self.black

                print(f"{game_ended} by {winner} in {self.fen.to_string()}")
                running = False
        
        if self.is_visual:
            pygame.quit()
        
        return win

    def get_move(self):
        time.sleep(self.delay_seconds)
        if self.fen.white_to_move():
            move = self.white.select_move(TIME_PER_MOVE_MILLI)

            if not self.fen.is_legal_move_lan(move):
                return False
        else:
            move = self.black.select_move(TIME_PER_MOVE_MILLI)

            if not self.fen.is_legal_move_lan(move):
                return False

        self.fen.lan_to_fen(move)
        self.white.apply_move(move)
        self.black.apply_move(move)
        print(f"Fen to {self.fen.to_string()} by move {move}")

        if self.is_visual:
            self.visual.place_piece(*lan_to_move(move), True)
            self.visual.update_fen_list()
        
        return True
    

class PlayerVsBotMatch:
    def __init__(self, bot, time_per_move_mili, player_is_white=True, fen=FenPy(), perspective=WHITE):
        self.player_is_white = player_is_white
        self.bot = bot
        self.time_per_move_mili = time_per_move_mili
        self.fen = fen
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
        print("Player vs bot match")

        self.visual.setup()
            
        running = True
        while running:
            move = 1
            if self.fen.white_to_move() and self.player_is_white:
                move = self.visual.get_move(WHITE)
            elif not self.fen.white_to_move() and not self.player_is_white:
                move = self.visual.get_move(BLACK)
            elif not self.visual.run_static(self):
                running = False

            if self.visual.fen != self.fen:
                self.fen = self.visual.fen
                continue

            if not move:
                running = False
            elif move != 1:
                print(f"Fen to {self.fen.to_string()} by move {move}")
                self.fen.lan_to_fen(move)
                self.bot.apply_move(move)

            game_ended = self.fen.game_ended()
            if game_ended == WHITE_WINS or game_ended == BLACK_WINS or game_ended == DRAW:
                print(self.fen.to_string())
                print(game_ended)
                running = False

        pygame.quit()
    
    def get_move(self):
        move = self.bot.select_move(self.time_per_move_mili)
        self.bot.apply_move(move)
        self.fen.lan_to_fen(move)
        print(f"Fen to {self.fen.to_string()} by move {move}")
        self.visual.place_piece(*lan_to_move(move), True)
        self.visual.update_fen_list()
        
        return True