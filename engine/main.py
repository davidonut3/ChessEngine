from config import *
from match import PlayerVsBotMatch, PlayerVsPlayerMatch, BotVsBotMatch
from testbot import TestBot
from visual.visual import Visual
from matches import run, bench

def perft(fen_str, per_move):
    for i in range(1,7):
        perft_check(i, fen_str, per_move)

def test():
    fen = FenPy()
    # fen = fen.from_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
    # fen = fen.from_str("8/8/3p4/1Pp4r/1K5k/5p2/4P1P1/1R6 w - c6 0 3")
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

# test()

perft("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", False)

# move_gen_perft_py()

# perft_check(4, "rnbqkbnr/1ppppppp/p7/8/3P4/8/PPPKPPPP/RNBQ1BNR b kq - 1 2", True) c7c5 en e7e5
# perft_check(2, "rnbqkbnr/1p1ppppp/p7/2P5/8/8/PPPKPPPP/RNBQ1BNR b kq - 0 3", True)