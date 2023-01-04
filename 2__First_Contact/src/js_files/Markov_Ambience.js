inlets = 1;
outlets = 8;

var value;
var outletVal;

autowatch = 1;

if (jsarguments.length > 1)
	value = jsarguments[1]; 

function random(x, y) {
    z = Math.floor(Math.random() * (y + 1)) + x;
    return z;
} 

function mkChain(prob1, prob2, prob3, prob4, prob5, prob6) {
    chanceValue = random(0, 100);
    if (chanceValue < prob1){
        return 0;
    } else if (chanceValue < prob2) {
        return 1;
    } else if (chanceValue < prob3) {
        return 2;
    } else if (chanceValue < prob4) {
        return 3;
    } else if (chanceValue < prob5) {
        return 4;
    } else if (chanceValue < prob6) {
        return 5;
    }
}

function msg_int(a) {
    value = a;
    var delay = random(200, 400);
    outlet(7, delay);
    switch(value) {
        case 0:
            outletVal = mkChain(10, 50, 65, 80, 90, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break;
        case 1:
            outletVal = mkChain(40, 50, 65, 80, 90, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break;
        case 2:
            outletVal = mkChain(10, 20, 30, 70, 85, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break;
        case 3:
            outletVal = mkChain(10, 20, 60, 70, 85, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break; 
        case 4:
            outletVal = mkChain(15, 30, 40, 50, 60, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break;
        case 5:
            outletVal = mkChain(15, 30, 40, 50, 90, 101);
            outlet(outletVal, 0);
            outlet(6, outletVal);
            break;
    }
}