inlets = 1;
outlets - 1;
autowatch = 1;

var newRhythm = new Array();

if (jsarguments.length > 1)
	newRhythm = jsarguments[1]; 
   
function random(x, y) {
    z = Math.floor(Math.random() * (y + 1)) + x;
    return z;
} 
//need to fix purple line here
function regenerate() {
    var rhythm = new Array();
    var blocks = new Array();
    //Generates 1st digit
    rhythm.push(random(0, 2));
    //Generates blueprint for 2-5 codeblocks
    switch(rhythm[0]) {
        //Generates blueprint for 3 beat blocks
        case 0:
            blocks.push(random(0, 1));
            switch(blocks[0]) {
                case 0:
                    blocks.push(random(0, 1));
                    if (blocks[1] == 0 ){
                        blocks.push(0);
                    }
                    break;
                case 1:
                    blocks.push(0);
                    break;
            }
            break;
        //Generates blueprint for 4 beat blocks
        case 1:
            blocks.push(random(0,1));
            blocks.push(random(0,1));
            b = blocks[0] + blocks[1];
            switch(b) {
                case 0:
                    blocks.push(random(0,1));
                    if (blocks[2] == 0) {
                        blocks.push(0);
                    }
                    break;
                case 1: 
                    blocks.push(0);
                    break;
                case 2:
                    break;
            }
            break;
        //Generates blueprint for 5 beat blocks
        case 2:
            blocks.push(random(0, 1));
            blocks.push(random(0, 1));
            z = blocks[0] + blocks[1]
            switch(z) {
                case 0:
                    blocks.push(random(0, 1));
                    switch(blocks[2]) {
                        case 0:
                            blocks.push(random(0, 1));
                            if (blocks[3] == 0) {
                                blocks.push(0);
                            }
                            break;
                        case 1:
                            blocks.push(0);
                            break;
                    }
                    break;
                case 1:
                    blocks.push(random(0, 1));
                    if (blocks[2] == 0) {
                        blocks.push(0)
                    }
                    break;
                case 2:
                    blocks.push(0);
                    break;
            }
            break;
    }
    //Generates digits 2-5 using blueprint from blocks
    for (var n = 0; n < blocks.length; n++){
        rhythm.push(blocks[n]);
        switch(blocks[n]) {
            case 0:
                newItem = random(0, 1);
                rhythm.push(newItem);
                switch(newItem) {
                    //red
                    case 0:
                        rhythm.push(0);
                        rhythm.push(0);
                        break;
                    //yellow
                    case 1:
                        newItem2 = random(0, 2);
                        rhythm.push(newItem2);
                        switch(newItem2) {
                            case 1:
                                newItem3 = random(0, 1);
                                rhythm.push(newItem3);
                                break;
                            default:
                                newItem3 = 0;
                                rhythm.push(newItem3);
                                break;
                        }
                        break;
                }
                break;
            case 1:
                newItem = random(0, 1);
                rhythm.push(newItem);
                switch(newItem) {
                    //purple
                    case 0:
                        newItem2 = random(0, 5);
                        rhythm.push(newItem2);
                        switch(newItem2) {
                            case 1, 2:
                                newItem3 = random(0,2);
                                rhythm.push(newItem3);
                                break;
                            default:
                                newItem3 = 0;
                                rhythm.push(newItem3);
                                break;
                        }
                        break;
                    //blue
                    case 1:
                        newItem2 = random(0, 2);
                        rhythm.push(newItem2);
                        switch(newItem2) {
                            case 1:
                                newItem3 = random(0, 1);
                                rhythm.push(newItem3);
                                break;
                            default:
                                newItem3 = 0;
                                rhythm.push(newItem3);
                                break;
                        }                   
                        break;
                }
                break;   
        }
    }
return rhythm;
}

function list() {
    newRhythm = arrayfromargs(arguments);
    outlet(0, newRhythm);
    //Determine position of value to replace
    var newPosition = random(1, newRhythm.length);
    //Determine which value(s) to replace based on position
    switch((newPosition - 1) % 4) {
        //replacing 2nd digit 
        case 0:
            previousValue = newRhythm[newPosition];
            newRhythm[newPosition] = random(0, 1);
            if (previousValue == newRhythm[newPosition]) {
                newRhythm[newPosition + 1] = random(0, 1);
                switch(newRhythm[newPosition + 1]) {
                    case 0:
                        switch(newRhythm[newPosition]) {
                            case 0:
                                newRhythm[newPosition + 2] = 0;
                                newRhythm[newPosition + 3] = 0;
                                break;
                            case 1:
                                newRhythm[newPosition + 2] = random(0, 3);
                                switch(newRhythm[newPosition + 2]) {
                                    case 1, 2:
                                        newRhythm[newPosition + 3] = random(0, 2);
                                        break;
                                    default:
                                        newRhythm[newPosition + 3] = 0;
                                }
                        }
                        break;
                    case 1:
                        newRhythm[newPosition + 2] + random(0, 2);
                        switch(newRhythm[newPosition + 2]) {
                            case 1:
                                newRhythm[newPosition + 3] = random(0, 1);
                                break;
                            default:
                                newRhythm[newPosition + 3] = 0;
                                break;
                        }
                        break;
                }
            } else {
                newRhythm = regenerate();
            }
            break;
        //replacing 3rd digit
        case 1:
            newRhythm[newPosition] = random(0, 1);
            switch(newRhythm[newPosition]) {
                case 0:
                    switch(newRhythm[newPosition - 1]) {
                        case 0:
                            newRhythm[newPosition + 1] = 0;
                            newRhythm[newPosition + 2] = 0;
                            break;
                        case 1:
                            newRhythm[newPosition + 1] = random(0, 3);
                            switch(newRhythm[newPosition + 1]) {
                                case 1, 2:
                                    newRhythm[newPosition + 2] = random(0, 2);
                                    break;
                                default:
                                    newRhythm[newPosition + 2] = 0;
                                    break;
                            }     
                            break;
                    }
                    break;
                case 1:
                    newRhythm[newPosition + 1] = random(0, 2);
                    switch(newRhythm[newPosition + 1]) {
                        case 1:
                            newRhythm[newPosition + 2] = random(0, 1);
                            break;
                        default:
                            newRhythm[newPosition + 2] = 0;
                            break;
                    }
                    break;
            }
            break; 
        //replacing 4th digit
        case 2:
            switch(newRhythm[newPosition - 1]) {
                case 0:
                    switch(newRhythm[newPosition -2]) {
                        case 0:
                            newRhythm[newPosition] = 0;
                            newRhythm[newPosition + 1] = 0;
                            break;
                        case 1:
                            newRhythm[newPosition] = random(0, 3);
                            switch(newRhythm[newPosition]) {
                                case 1,2:
                                    newRhythm[newPosition + 1] = random(0, 2);
                                    break;
                                default:
                                    newRhythm[newPosition + 1] = 0;
                                    break;
                            }
                            break;
                    }
                    break;
                case 1:
                    newRhythm[newPosition] = random(0, 2);
                    switch(newRhythm[newPosition]) {
                        case 1:
                            newRhythm[newPosition + 1] = random(0, 1);
                            break;
                        default:
                            newRhythm[newPosition + 1] = 0;
                            break;
                    }
                    break;
            }
            break;
        //replacing 5th digit 
        case 3:
            switch(newRhythm[newPosition - 3]) {
                case 0:
                    switch(newRhythm[newPosition - 2]) {
                        case 0:
                            //red line
                            newRhythm[newPosition - 1] = 0;
                            newRhythm[newPosition] = 0;
                            break;
                        case 1:
                            //yelow line
                            switch(newRhythm[newPosition - 1]) {
                                case 1:
                                    newRhythm[newPosition] = random(0, 1);
                                    break;
                                default:
                                    newRhythm[newPosition] = 0;
                                    break;
                            }
                            break;
                    }
                    break;
                case 1:
                    switch(newPosition - 2){
                        case 0:
                            //purple line
                            switch(newRhythm[newPosition - 1]) {
                                case 1, 2:
                                    newRhythm[newPosition] = random(0, 2);
                                    break;
                                default:
                                    newRhythm[newPosition]
                            }
                        case 1:
                            //blue line
                            switch(newPosition - 1) {
                                case 1:
                                    newRhythm[newPosition] = random(0, 1);
                                    break;
                                default:
                                    newRhythm[newPosition] = 0;
                                    break;
                            }
                    }
                    break;
            }
            break;
    }
    for (i = 0; i < newRhythm.lengths; i++) {
        if (typeof newRhythm[i] == undefined) {
            newRhythm = regenerate();
            post("Bruh");
        }
    }
    //range solution
    outlet(0, newRhythm);
    newRhythm.length = 0;
}