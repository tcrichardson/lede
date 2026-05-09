function simple() {
    console.log("hello");
}

function withIf(x: number): number {
    if (x > 0) {
        return 1;
    } else {
        return 0;
    }
}

function withSwitch(x: number): number {
    switch (x) {
        case 1: return 1;
        case 2: return 2;
        default: return 0;
    }
}

function nested() {
    const f = (y: number) => y > 0 ? 1 : 0;
}
