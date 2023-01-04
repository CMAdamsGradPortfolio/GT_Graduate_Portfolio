inlets = 1;
outlets = 4;

var pitchList = new Array();
var rhythmList = new Array();
var pitchOut = new Array();
var rhythmOut = new Array();
var measureOut = new Array();

//keep in mind this is only generating 1 event, need to figure out how to both bach.beatbox along to generate each event

autowatch = 1;

if (jsarguments.length > 1)
	a = jsarguments[1]; 

//Need to change from assigning values to .push
function manifestRhythm(digit2, digit3, digit4, digit5) {
    switch(digit2) {
        case 0:
            switch(digit3) {
                //red line
                case 0:
                    rhythmOut.push("1/12", "1/12", "1/12");
                    break;
                //yellow line
                case 1:
                    switch(digit4) {
                        case 0:
                            rhythmOut.push("1/8", "1/8");
                            break;
                        case 1:
                            switch(digit5) {
                                case 0:
                                    rhythmOut.push("1/16", "1/16", "1/8");
                                    break;
                                case 1:
                                    rhythmOut.push("1/8", "1/16", "1/16");
                                    break;                                
                            }
                            break;
                        case 2:
                            rhythmOut.push("1/16", "1/16", "1/16", "1/16");
                            break;
                    }
                    break;
            }
            break;
        case 1:
            switch(digit3) {
                //purple line
                case 0:
                    switch(digit4) { 
                        case 0:
                            rhythmOut.push("1/6", "1/6", "1/6");
                            break;
                        case 1:
                            switch(digit5) {
                                case 0:
                                    rhythmOut.push("1/12", "1/12", "1/6", "1/6");
                                    break;
                                case 1:
                                    rhythmOut.push("1/6", "1/12", "1/12", "1/6");
                                    break;
                                case 2:
                                    rhythmOut.push("1/6", "1/6", "1/12", "1/12");
                                    break;
                            }
                            break;
                        case 2:
                            switch(digit5) {
                                case 0:
                                    rhythmOut.push("1/6", "1/12", "1/12", "1/12", "1/12");
                                    break;
                                case 1:
                                    rhythmOut.push("1/12", "1/12", "1/6", "1/12", "1/12");
                                    break;
                                case 2:
                                    rhythmOut.push("1/12", "1/12", "1/12", "1/12", "1/6");
                                    break;
                            }
                            break;
                        case 3:
                            rhythmOut.push("1/12", "1/12", "1/12", "1/12", "1/12", "1/12");
                            break;
                    }
                    break;
                //blue line 
                case 1:
                    switch(digit4) {
                        case 0:
                            rhythmOut.push("1/4", "1/4");
                            break;       
                        case 1:
                            switch(digit5) {
                                case 0:
                                    rhythmOut.push("1/8", "1/8", "1/4");
                                    break;
                                case 1:
                                    rhythmOut.push("1/4", "1/8", "1/8");
                                    break;                               
                            }
                            break;
                        case 2:
                            rhythmOut.push("1/8", "1/8", "1/8", "1/8");
                            break;
                    }
                    break;
            }
            break;
    }
}
//convert pitch values to bach values
function manifestPitch(rhythmLength) {
    for (var i = 0; i < rhythmLength; i++) {
        switch (pitchList[i]) {
            case 0:
                pitchOut.push("C5");
                break;
            case 1:
                pitchOut.push("C#5");
                break;
            case 2:
                pitchOut.push("D5");
                break;
            case 3:
                pitchOut.push("D#5");
                break;
            case 4:
                pitchOut.push("E5");
                break;
            case 5:
                pitchOut.push("F5");
                break;
            case 6:
                pitchOut.push("F#5");
                break;
            case 7:
                pitchOut.push("G5");
                break;
            case 8:
                pitchOut.push("G#5");
                break;
            case 9:
                pitchOut.push("A5");
                break;
            case 10:
                pitchOut.push("A#5");
                break;
            case 11:
                pitchOut.push("B5");
                break;
            default:
                pitchOut.push("C5");
                break;    
        }
    }
    return pitchOut
}

//main function
function list() {
	var a = arrayfromargs(arguments);
    for (var i = 0; i < 10; i++) {
        pitchList[i] = a[i];
    }
    for (var i = 0; i < a.length - 10; i++) {
        rhythmList[i] = a[i + 10];
    }
    switch(rhythmList[0]) {
        case 0:
            measureOut[0] = 3;
            measureOut[1] = 4;
            manifestRhythm(rhythmList[1], rhythmList[2], rhythmList[3], rhythmList[4]);    
            break;
        case 1:
            measureOut[0] = 4;
            measureOut[1] = 4;
            manifestRhythm(rhythmList[1], rhythmList[2], rhythmList[3], rhythmList[4]);
            manifestRhythm(rhythmList[5], rhythmList[6], rhythmList[7], rhythmList[8]);
            break;
        case 2:
            measureOut[0] = 5;
            measureOut[1] = 4;
            manifestRhythm(rhythmList[1], rhythmList[2], rhythmList[3], rhythmList[4]);
            manifestRhythm(rhythmList[5], rhythmList[6], rhythmList[7], rhythmList[8]);
            manifestRhythm(rhythmList[9], rhythmList[10], rhythmList[11], rhythmList[12]);
            break;
    }
    outlet(2, manifestPitch(rhythmOut.length));
    outlet(3, rhythmOut);
    outlet(0, measureOut);
    outlet(1, 0);
    pitchList.length = 0;
    rhythmList.length = 0;
    pitchOut.length = 0;
    rhythmOut.length = 0;
    measureOut.length = 0;
}