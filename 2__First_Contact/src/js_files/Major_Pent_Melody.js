inlets = 1;
oulets = 1;

var b = new Array();

autowatch = 1;

if (jsarguments.length > 1)
	a = jsarguments[1]; 

function list() {
	var a = arrayfromargs(arguments);
	b.push(parseInt(a[0], 10));
	b.push(parseInt(a[1], 10));	
	var x = b[0];
	var y = b[1];
	a.length = 0;
	b.length = 0;
	switch (x) {
		case 1:
			switch (y) {
				case 0:
					outlet(0, 0);
					break;
				case 1:
					outlet(0, 4);
					break;
				case 2:
					outlet(0, 9);
					break;
				}
			break;
		case 2: 
			switch (y) {
				case 0:
					outlet(0, 2);
					break;
				case 1:
					outlet(0, 7);
					break;
				case 2:
					outlet(0, 0);
					break;
				}
			break;
		case 3: 
			switch (y) {
				case 0:
					outlet(0, 4);
					break;
				case 1:
					outlet(0, 9);
					break;
				case 2:
					outlet(0, 2);
					break;
				}
			break;
		case 4:
			switch (y) {
				case 0:
					outlet(0, 7);
					break;
				case 1:
					outlet(0, 0);
					break;
				case 2:
					outlet(0, 4);
					break;
				}
			break;
		case 5:
			switch (y) {
				case 0:
					outlet(0, 9);
					break;
				case 1:
					outlet(0, 2);
					break;
				case 2:
					outlet(0, 7);
					break;
				}
			break;
		case 6:
			switch (y) {
				case 0:
					outlet(0, 0);
					break;
				case 1:
					outlet(0, 4);
					break;
				case 2:
					outlet(0, 7);
					break;
				}
			break;
		case 7:
			switch (y) {
				case 0:
					outlet(0, 2);
					break;
				case 1:
					outlet(0, 7);
					break;
				case 2:
					outlet(0, 9);
					break;
				}
			break;
		default:
			outlet(0, y);
			post(x);
			post(y);
			break;
	}
}		
