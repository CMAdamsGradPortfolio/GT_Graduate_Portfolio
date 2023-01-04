inlets = 1;
oulets = 1;

var modifiers = new Array();
var noteList = new Array();

autowatch = 1;

if (jsarguments.length > 1)
	a = jsarguments[1]; 
    
function list() {
	var a = arrayfromargs(arguments);
	modifiers.push(parseInt(a[0], 10));
	modifiers.push(parseInt(a[1], 10));
    for (var i = 2; i < a.length; i++) {
        noteList.push(parseInt(a[i], 10));
    }
    outlet(0, noteList);
    noteList.splice(modifiers[0], 1, modifiers[1]);
    outlet(0, noteList);
    modifiers.length = 0;
    noteList.length = 0;
}