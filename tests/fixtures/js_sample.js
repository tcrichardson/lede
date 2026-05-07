function simple() {
    console.log("hello");
}

function withIf(x) {
    if (x > 0) {
        console.log("pos");
    } else {
        console.log("non-pos");
    }
}

function withSwitch(x) {
    switch (x) {
        case 1: break;
        case 2: break;
        default: break;
    }
}

function nested() {
    const f = (y) => y > 0 ? 1 : 0;
}
