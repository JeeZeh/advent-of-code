def nic():
    x = 1
    while x != 0:
        if x < 0: 
            print("OUTPUT")
            yield x
            print("FLIPPING")
            x*= -1
        if x > 0:
            print("EXPECTING")

            x = yield
            print("SAVED")

a = nic()

a.send(None)

a.send(5)
a.send(-10)
a.send(20)
a.send(-10)
a.send(20)