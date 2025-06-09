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
    # fen = fen.from_str("8/k6p/2p4p/8/8/8/8/4Q2K w - - 0 69")
    bot1 = BotV1_2Py()
    bot2 = BotV1_1Py()
    bot1 = bot1.from_fen(fen.to_string())
    bot2 = bot2.from_fen(fen.to_string())
    # PlayerVsBotMatch(bot1, player_is_white=True, fen=fen, perspective=BLACK).run_match()
    PlayerVsPlayerMatch(fen, WHITE).run_match()
    # BotVsBotMatch(bot1, bot2, delay=0, fen=fen, is_visual=True, perspective=WHITE).run_match()

# if __name__ == "__main__":
#     bench()

test()