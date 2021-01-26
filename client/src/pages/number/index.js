import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK, KEY_DOWN, KEY_UP } from "../common/CommandCode.js";
import {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    DELETE, EQUAL, SLASH, MINUS, RETURN,
    SHIFT, COMMAND,
    X, C, V, COMMA, PERIOD,
    UP_ARROW, RIGHT_ARROW, DOWN_ARROW, LEFT_ARROW,
} from "../common/Key.js";
import createButton from "../common/createButton.js";

const container = document.getElementById("container")
const s = new SocketManager(container);

container.ontouchstart = () => {
    return false;
};

createButton(s, "delete", new Uint8Array([KEY_CLICK, DELETE]));
createButton(s, "equal", new Uint8Array([KEY_CLICK, EQUAL]));
createButton(s, "slash", new Uint8Array([KEY_CLICK, SLASH]));
createButton(s, "asterisk", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N8, KEY_UP, SHIFT]));
createButton(s, "seven", new Uint8Array([KEY_CLICK, N7]));
createButton(s, "eight", new Uint8Array([KEY_CLICK, N8]));
createButton(s, "nine", new Uint8Array([KEY_CLICK, N9]));
createButton(s, "minus", new Uint8Array([KEY_CLICK, MINUS]));
createButton(s, "four", new Uint8Array([KEY_CLICK, N4]));
createButton(s, "five", new Uint8Array([KEY_CLICK, N5]));
createButton(s, "six", new Uint8Array([KEY_CLICK, N6]));
createButton(s, "plus", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, EQUAL, KEY_UP, SHIFT]));
createButton(s, "one", new Uint8Array([KEY_CLICK, N1]));
createButton(s, "two", new Uint8Array([KEY_CLICK, N2]));
createButton(s, "three", new Uint8Array([KEY_CLICK, N3]));
createButton(s, "return", new Uint8Array([KEY_CLICK, RETURN]));
createButton(s, "zero", new Uint8Array([KEY_CLICK, N0]));
createButton(s, "period", new Uint8Array([KEY_CLICK, PERIOD]));

createButton(s, "cut", new Uint8Array([KEY_DOWN, COMMAND, KEY_CLICK, X, KEY_UP, COMMAND]));
createButton(s, "copy", new Uint8Array([KEY_DOWN, COMMAND, KEY_CLICK, C, KEY_UP, COMMAND]));
createButton(s, "paste", new Uint8Array([KEY_DOWN, COMMAND, KEY_CLICK, V, KEY_UP, COMMAND]));
createButton(s, "leftparen", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N9, KEY_UP, SHIFT]));
createButton(s, "up", new Uint8Array([KEY_CLICK, UP_ARROW]));
createButton(s, "rightparen", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N0, KEY_UP, SHIFT]));
createButton(s, "left", new Uint8Array([KEY_CLICK, LEFT_ARROW]));
createButton(s, "percent", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N5, KEY_UP, SHIFT]));
createButton(s, "right", new Uint8Array([KEY_CLICK, RIGHT_ARROW]));
createButton(s, "less", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, COMMA, KEY_UP, SHIFT]));
createButton(s, "down", new Uint8Array([KEY_CLICK, DOWN_ARROW]));
createButton(s, "greater", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, PERIOD, KEY_UP, SHIFT]));
createButton(s, "caret", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N6, KEY_UP, SHIFT]));
createButton(s, "dollar", new Uint8Array([KEY_DOWN, SHIFT, KEY_CLICK, N4, KEY_UP, SHIFT]));
createButton(s, "comma", new Uint8Array([KEY_CLICK, COMMA]));
