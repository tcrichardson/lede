def simple():
    print("hello")

def with_if(x):
    if x > 0:
        print("pos")
    else:
        print("non-pos")

def with_match(x):
    match x:
        case 1:
            print("one")
        case 2:
            print("two")
        case _:
            print("other")

def nested():
    f = lambda y: 1 if y > 0 else 0
