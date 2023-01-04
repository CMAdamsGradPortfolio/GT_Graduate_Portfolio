inlets = 1;
outlets = 1;
autowatch = 1;

var productSlice1 = 1;
var productSlice2 = 1;

if (jsarguments.length > 1)
	a = jsarguments[1]; 

function list() {
    a = arrayfromargs(arguments);
    for (i = 0; i < (a.length/2); i++) {
        productSlice1 *= a[i]; 
    }
    for (i = (a.length/2); i < a.length; i++) {
        productSlice2 *= a[i];
    }
    var difference = 0;
    difference = Math.abs(productSlice1 - productSlice2);
    outlet(0, difference);
}