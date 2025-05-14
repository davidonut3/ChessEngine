from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual
from matches import run, bench

def test():
    fen = FenPy()
    # fen = fen.from_str('6k1/5ppp/8/8/8/8/5PPP/5qK1 w - - 0 1')
    bot1 = BotV1Py()
    bot2 = BotV1Py()
    bot1 = bot1.from_fen(fen.to_string())
    bot2 = bot2.from_fen(fen.to_string())
    PlayerVsBotMatch(bot1, player_is_white=True, fen=fen, perspective=WHITE).run_match()
    # BotVsBotMatch(visual_debugger, bot1, bot2, delay=0, fen=fen, is_visual=True, perspective=WHITE).run_match()
    # PlayerVsPlayerMatch(fen, WHITE).run_match()

# def show_board(board):
#     fen = FenPy().from_str(board)
#     visual = Visual(fen, WHITE)
#     visual.setup()
#     visual.show_board()

# show_board('8/B5R1/8/2p2P2/2P5/p5pb/1kq2b2/4K3 w - - 14 99')

# if __name__ == "__main__":
#     bench()

# test()

perft_check(3, DEFAULT)