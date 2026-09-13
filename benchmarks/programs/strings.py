# String formatting + concatenation churn. %-format / join / len.
parts = []
acc = 0
for i in range(1, 200001):
    s = "item-%d-%x" % (i, i * 7)
    parts.append(s)
    acc += len(s)
big = ",".join(parts)
print("%d" % ((acc + len(big)) % 1000000007))
