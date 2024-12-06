from graphlib import TopologicalSorter


rules, updates = open("test_input.txt").read().split("\n\n")
target_source_graph = {}

# create graph
for rule in rules.split("\n"):
    source, target = rule.split("|")
    if target in target_source_graph:
        target_source_graph[target].append(source)
    else:
        target_source_graph[target] = [source]


part1_total, part2_total = 0, 0
for update in updates.split("\n"):
    update = update.split(",")

    # find rules applicable to this update by trimming graph.
    # this is necessary because of cycles in the graph
    this_graph = {}
    for page in update:
        if page in target_source_graph:
            this_graph[page] = target_source_graph[page]
    ts = TopologicalSorter(this_graph)
    rule_order = list(ts.static_order())

    valid = True
    rule_order_index = 0
    for i, page in enumerate(update):
        index_of_page = rule_order.index(page)
        if index_of_page < rule_order_index:
            valid = False
            break
        rule_order_index = index_of_page
    if valid:
        sorted_update = update
        middle = update[len(update) // 2]
        part1_total += int(middle)
    if not valid:
        sorted_update = list(filter(lambda x: x in update, rule_order))
        middle = sorted_update[len(sorted_update) // 2]
        part2_total += int(middle)

print("part1", part1_total)
print("part2", part2_total)
