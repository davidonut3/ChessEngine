from config import *
from visual.utils import *
from visual.visual import Visual


class PlayerVsPlayerMatch:
    def __init__(self, fen=FenPy(), perspective=WHITE):
        self.fen = fen
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
        self.visual.setup()
            
        running = True
        while running:
            if self.fen.white_to_move():
                move = self.visual.get_move(WHITE)
            else:
                move = self.visual.get_move(BLACK)
            
            if not move:
                running = False
            else:
                self.fen.lan_to_fen(move)

            game_ended = self.fen.game_ended()
            if game_ended == '1-0' or game_ended == '0-1' or game_ended == '½-½':
                print(WIN[game_ended])
                running = False

        pygame.quit()


class BotVsBotMatch:
    def __init__(self, white, black, delay=0, fen=FenPy(), is_visual=True, perspective=WHITE):
        self.white = white
        self.black = black
        self.delay = delay
        self.fen = fen
        self.is_visual = is_visual or self.white == USER or self.black == USER
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
        win = 'not ended'

        if self.is_visual:
            self.visual.setup()

        running = True
        while running:
            if self.is_visual:
                if not self.visual.run_static(self):
                    running = False
            else:
                self.get_move()

            game_ended = self.fen.game_ended()
            if game_ended == '1-0' or game_ended == '0-1' or game_ended == '½-½':
                win = game_ended
                print(WIN[win])
                running = False
        
        if self.is_visual:
            pygame.quit()
        
        return win

    def get_move(self):
        time.sleep(self.delay)
        if self.fen.white_to_move():
            move = self.white.get_move()
            self.fen.lan_to_fen(move)
            self.black.receive_move(move)
        else:
            move = self.black.get_move()
            self.fen.lan_to_fen(move)
            self.white.receive_move(move)

        if self.is_visual:
            self.visual.place_piece(*lan_to_move(move), True)
        
        return True
    

class PlayerVsBotMatch:
    def __init__(self, bot, player_is_white=True, fen=FenPy(), perspective=WHITE):
        self.player_is_white = player_is_white
        self.bot = bot
        self.fen = fen
        self.perspective = perspective
        self.visual = Visual(self.fen, self.perspective)

    def run_match(self):
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

            if not move:
                running = False
            elif move != 1:
                print(f"Fen to {self.fen.to_string()} by move {move}")
                self.fen.lan_to_fen(move)
                self.bot.receive_move(move)

            game_ended = self.fen.game_ended()
            if game_ended == '1-0' or game_ended == '0-1' or game_ended == '½-½':
                print(self.fen.to_string())
                print(WIN[game_ended])
                running = False

        pygame.quit()
    
    def get_move(self):
        move = self.bot.get_move()
        self.fen.lan_to_fen(move)
        print(f"Fen to {self.fen.to_string()} by move {move}")
        self.visual.place_piece(*lan_to_move(move), True)
        
        return True