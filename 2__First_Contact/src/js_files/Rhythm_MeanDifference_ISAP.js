inlets = 1;
outlets = 1;

var recievedList = new Array();
var occurences = new Array();

autowatch = 1;

if (jsarguments.length > 1)
	a = jsarguments[1]; 

function numberOfOccurences(recievedList) {
    var number0s = 0;
    var number1s = 0;
    var otherNums = 0;
    for (i = 1; i < recievedList.length; i++) {
        switch(recievedList[i]) {
            case 0:
                number0s += 1;
                break;
            case 1:
                number1s += 1;
                break;
            default:
                otherNums += 1;
                break;
        }      
    }
    occurences[0] = number0s;
    occurences[1] = number1s;
    occurences[2] = otherNums;
    return occurences;
}

function list() {
    a = arrayfromargs(arguments);
    b = numberOfOccurences(a);
    var difference1 = 0;
    var difference2 = 0;
    var difference3 = 0;
    var meanDifference = 0;
    difference1 = Math.abs((b[0] * b[1]) - (b[0] * b[2]));
    difference2 = Math.abs((b[1] * b[0]) - (b[1] * b[2]));
    difference3 = Math.abs((b[2] * b[1]) - (b[2] * b[0]));
    meanDifference = (difference1 + difference2 + difference3)/3.0;
    outlet(0, meanDifference);
}
