from typing import List, Tuple
import random
import time
time_start = time.time()

N = 1000  # 注文の個数（1000 で固定）


class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x: int = x
        self.y: int = y

    def __str__(self) -> str:
        # f文字列は PyPy（競プロでよく使われる Python の高速版）では使えないはず
        return '{} {}'.format(self.x, self.y)


class Order:
    def __init__(self, rest: Point, house: Point, id_: int) -> None:
        self.rest: Point = rest
        self.house: Point = house
        self.id: int = id_


def get_dist(p0: Point, p1: Point) -> int:
    return abs(p0.x-p1.x) + abs(p0.y-p1.y)


# TODO: スコア計算用の関数を用意
def calc_score():
    pass


# 注文 [0,49] を「目的地のうち一番近いところに行く」貪欲法で解く
def make_0_49_greedy_ans(orders: List[Order]) -> Tuple[List[int], List[Point]]:
    orders50 = orders[:50]
    ans_inds = list(range(50))
    ans_points = []

    # (400, 400) からスタート
    curr_point = Point(400, 400)
    ans_points.append(curr_point)

    # レストラン 50 件を貪欲法で回る
    INF = 10**10
    visited = set()
    for _ in range(50):
        min_id, min_point, min_dist = None, None, INF
        for order in orders50:
            if order.id in visited:
                continue
            dist = get_dist(order.rest, curr_point)
            if dist < min_dist:
                min_id, min_point, min_dist = order.id, order.rest, dist
        visited.add(min_id)
        ans_points.append(min_point)
        curr_point = min_point

    # 家 50 件を貪欲法で回る
    visited = set()
    for _ in range(50):
        min_id, min_point, min_dist = None, None, INF
        for order in orders50:
            if order.id in visited:
                continue
            dist = get_dist(order.house, curr_point)
            if dist < min_dist:
                min_id, min_point, min_dist = order.id, order.house, dist
        visited.add(min_id)
        ans_points.append(min_point)
        curr_point = min_point

    # (400, 400) に戻る
    curr_point = Point(400, 400)
    ans_points.append(curr_point)

    return (ans_inds, ans_points)


def main():
    orders: list[Order] = []
    for i in range(N):
        a, b, c, d = map(int, input().split())
        order = Order(rest=Point(a, b), house=Point(c, d), id_=i)
        orders.append(order)

    ans_ids, ans_points = make_0_49_greedy_ans(orders)

    TIME_LIMIT = 1700  # ms
    while (time.time() - time_start)*1000 < TIME_LIMIT:
        # 制限時間いっぱいを使って、解を改善する
        # ランダムで 2 つの地点を入れ替えてみて、スコアが改善されるか確認するなど...
        pass

    # 答えの出力
    print(len(ans_ids), end=' ')
    print(*map(lambda i: i+1, ans_ids))  # 注文の id は 1-indexed で出力することに注意
    print(len(ans_points), end=' ')
    print(*ans_points)


if __name__ == '__main__':
    main()
