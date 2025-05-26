from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual
from matches import run, bench

def perft(fen_str, per_move):
    for i in range(1,9):
        perft_check(i, fen_str, per_move)

def test():
    fen = FenPy()
    fen = fen.from_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
    fen = fen.from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ")
    bot1 = BotV1Py()
    bot2 = BotV1Py()
    bot1 = bot1.from_fen(fen.to_string())
    bot2 = bot2.from_fen(fen.to_string())
    # PlayerVsBotMatch(bot1, player_is_white=True, fen=fen, perspective=WHITE).run_match()
    # BotVsBotMatch(visual_debugger, bot1, bot2, delay=0, fen=fen, is_visual=True, perspective=WHITE).run_match()
    PlayerVsPlayerMatch(fen, WHITE).run_match()

# def show_board(board):
#     fen = FenPy().from_str(board)
#     visual = Visual(fen, WHITE)
#     visual.setup()
#     visual.show_board()

# show_board('8/B5R1/8/2p2P2/2P5/p5pb/1kq2b2/4K3 w - - 14 99')

# if __name__ == "__main__":
#     bench()

test()

# perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ", False)

# move_gen_perft_py()

# perft_check(2, "8/2p5/3p4/1P5r/1K5k/5p2/4P1P1/1R6 b - - 1 2", True)