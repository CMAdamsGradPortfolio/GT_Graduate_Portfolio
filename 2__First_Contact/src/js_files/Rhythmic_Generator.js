inlets = 1;
oulets = 1;
autowatch = 1;

if (jsarguments.length > 1)
	numberToGenerate = jsarguments[1]; 
   
function random(x, y) {
    z = Math.floor(Math.random() * (y + 1)) + x;
    return z;
} 

//main function, generates a rhythmic pop upon bang
function bang() {
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

    outlet(0, rhythm);
    rhythm.length = 0;
    blocks.length = 0;
}