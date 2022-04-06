from typing import List, Tuple
import random
import time
import sys
time_start = time.time()

N = 1000  # 注文の個数（1000 で固定）


# 座標
class Point:
    def __init__(self, x: int, y: int) -> None:
        self.x: int = x
        self.y: int = y

    def __str__(self) -> str:
        # f文字列は PyPy（競プロでよく使われる Python の高速版）では使えないはず
        return '{} {}'.format(self.x, self.y)

    def __eq__(self, other) -> bool:
        return self.x == other.x and self.y == other.y

    def __hash__(self) -> int:
        return self.x * 10**8 + self.y


# 注文
class Order:
    def __init__(self, rest: Point, house: Point, id_: int) -> None:
        self.rest: Point = rest
        self.house: Point = house
        self.id: int = id_


# 座標（Point）間の距離を求める
def get_dist(p0: Point, p1: Point) -> int:
    return abs(p0.x-p1.x) + abs(p0.y-p1.y)


# 注文 [0,49] を「目的地のうち一番近いところに行く」貪欲法で解く
# List[注文のid], List[移動する座標] を返す
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


# スコア計算用の関数（スコアが合う以外はちゃんとテストしてないです）
# 必要に応じて、（ブラックボックスとして）使用してください。
def calc_score(orders: List[Order], ans_order_ids: List[int], ans_points: List[Point]) -> int:
    if not ans_points:
        print('ans_points is empty', file=sys.stderr)
        return 0
    if len(ans_order_ids) != 50:
        print('Warning: ans_order_ids len is not 50.', file=sys.stderr)
    if ans_points[0] != Point(400, 400) or ans_points[-1] != Point(400, 400):
        print('ans_points[0] and ans_points[-1] must be Point(400,400).', file=sys.stderr)
        return 0

    point_to_ids = {}
    for order in orders:
        point_to_ids.setdefault(order.rest, ([], []))
        point_to_ids[order.rest][0].append(order.id)
        point_to_ids.setdefault(order.house, ([], []))
        point_to_ids[order.house][1].append(order.id)

    pending_st = set(ans_order_ids)
    picked_st = set()
    if len(pending_st) != len(ans_order_ids):
        print('some order ids are duplicated.', file=sys.stderr)
        return 0

    for p in ans_points:
        if not p in point_to_ids:
            continue
        rest_ids = point_to_ids[p][0]
        for r_id in rest_ids:
            if r_id in pending_st:
                pending_st.remove(r_id)
                picked_st.add(r_id)
        house_ids = point_to_ids[p][1]
        for h_id in house_ids:
            if h_id in picked_st:
                picked_st.remove(h_id)

    if pending_st or picked_st:
        print('some orders have not been done.', file=sys.stderr)
        return 0

    dist = sum([get_dist(ans_points[i], ans_points[i+1]) for i in range(len(ans_points)-1)])
    score = round(10**8/(1000+dist))
    return score


def main():
    orders: list[Order] = []
    for i in range(N):
        a, b, c, d = map(int, input().split())
        order = Order(rest=Point(a, b), house=Point(c, d), id_=i)
        orders.append(order)

    ans_order_ids, ans_points = make_0_49_greedy_ans(orders)

    TIME_LIMIT = 1700  # ms
    while (time.time() - time_start)*1000 < TIME_LIMIT:
        # 制限時間いっぱいを使って、解を改善可能
        # ランダムで 2 つの地点を入れ替えてみて、スコアが改善されるか確認するなど...
        pass

    score = calc_score(orders, ans_order_ids, ans_points)
    print('score:', score, file=sys.stderr)

    # 答えの出力
    print(len(ans_order_ids), end=' ')
    print(*map(lambda i: i+1, ans_order_ids))  # 注文の id は 1-indexed で出力することに注意
    print(len(ans_points), end=' ')
    print(*ans_points)


if __name__ == '__main__':
    main()
