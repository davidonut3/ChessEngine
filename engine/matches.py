from config import *
from random_games import games
from match import BotVsBotMatch

def bench():
    start = time.time()
    bot1_wins, bot2_wins, draws = run()
    print(f'Bot 1 won {bot1_wins} games, bot 2 won {bot2_wins} games, and the bots drew {draws} times.')
    print(time.time() - start)

def run_match_worker(games):
    results = []

    for game in games:
        fen = FenPy()
        fen = fen.from_str(game)

        bot1 = BotV1Py().from_fen(fen.to_string())
        bot2 = BotV1Py().from_fen(fen.to_string())
        
        match = BotVsBotMatch(None, bot1, bot2, delay=0, fen=fen, is_visual=False, perspective=WHITE)
        result = match.run_match()
        results.append(result)

    return results

def run():
    sampled_games = random.sample(games, GAMES)
    batches = [sampled_games[i:i + BATCH_SIZE] for i in range(0, GAMES, BATCH_SIZE)]

    bot1_wins = 0
    bot2_wins = 0
    draws = 0

    with multiprocessing.Pool(processes=len(batches)) as pool:
        all_results = pool.map(run_match_worker, batches)

    for result_list in all_results:
        for result in result_list:
            if result == '1-0':
                bot1_wins += 1
            elif result == '0-1':
                bot2_wins += 1
            elif result == '½-½' or result == '1/2-1/2':
                draws += 1
            else:
                print(f"Unknown result: {result}")

    return bot1_wins, bot2_wins, draws