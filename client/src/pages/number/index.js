import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK, KEY_CLICK_FLAGS } from "../common/CommandCode.js";
import {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    DELETE, EQUAL, SLASH, MINUS, RETURN,
    X, C, V, COMMA, PERIOD,
    UP_ARROW, RIGHT_ARROW, DOWN_ARROW, LEFT_ARROW
} from "../common/Key.js";
import { SHIFT, COMMAND } from "../common/Flags.js";
import { createDownButton, createDownUpCustomButton, createDownButtonElem } from "../common/createButton.js";

const container = document.getElementById("container")
const s = new SocketManager(container);

container.ontouchstart = () => {
    return false;
};

createDownButton(s, "delete", new Uint8Array([KEY_CLICK, DELETE]));
createDownButton(s, "equal", new Uint8Array([KEY_CLICK, EQUAL]));
createDownButton(s, "slash", new Uint8Array([KEY_CLICK, SLASH]));
createDownButton(s, "asterisk", new Uint8Array([KEY_CLICK_FLAGS, N8, SHIFT]));
createDownButton(s, "seven", new Uint8Array([KEY_CLICK, N7]));
createDownButton(s, "eight", new Uint8Array([KEY_CLICK, N8]));
createDownButton(s, "nine", new Uint8Array([KEY_CLICK, N9]));
createDownButton(s, "minus", new Uint8Array([KEY_CLICK, MINUS]));
createDownButton(s, "four", new Uint8Array([KEY_CLICK, N4]));
createDownButton(s, "five", new Uint8Array([KEY_CLICK, N5]));
createDownButton(s, "six", new Uint8Array([KEY_CLICK, N6]));
createDownButton(s, "plus", new Uint8Array([KEY_CLICK_FLAGS, EQUAL, SHIFT]));
createDownButton(s, "one", new Uint8Array([KEY_CLICK, N1]));
createDownButton(s, "two", new Uint8Array([KEY_CLICK, N2]));
createDownButton(s, "three", new Uint8Array([KEY_CLICK, N3]));
createDownButton(s, "return", new Uint8Array([KEY_CLICK, RETURN]));
createDownButton(s, "comma", new Uint8Array([KEY_CLICK, COMMA]));
createDownButton(s, "zero", new Uint8Array([KEY_CLICK, N0]));
createDownButton(s, "period", new Uint8Array([KEY_CLICK, PERIOD]));

createDownButton(s, "cut", new Uint8Array([KEY_CLICK_FLAGS, X, COMMAND]));
createDownButton(s, "copy", new Uint8Array([KEY_CLICK_FLAGS, C, COMMAND]));
createDownButton(s, "paste", new Uint8Array([KEY_CLICK_FLAGS, V, COMMAND]));
createDownButton(s, "leftparen", new Uint8Array([KEY_CLICK_FLAGS, N9, SHIFT]));
createDownButton(s, "up", new Uint8Array([KEY_CLICK, UP_ARROW]));
createDownButton(s, "rightparen", new Uint8Array([KEY_CLICK_FLAGS, N0, SHIFT]));
createDownButton(s, "left", new Uint8Array([KEY_CLICK, LEFT_ARROW]));
createDownButton(s, "percent", new Uint8Array([KEY_CLICK_FLAGS, N5, SHIFT]));
createDownButton(s, "right", new Uint8Array([KEY_CLICK, RIGHT_ARROW]));
createDownButton(s, "less", new Uint8Array([KEY_CLICK_FLAGS, COMMA, SHIFT]));
createDownButton(s, "down", new Uint8Array([KEY_CLICK, DOWN_ARROW]));
createDownButton(s, "greater", new Uint8Array([KEY_CLICK_FLAGS, PERIOD, SHIFT]));
createDownButton(s, "caret", new Uint8Array([KEY_CLICK_FLAGS, N6, SHIFT]));
createDownButton(s, "dollar", new Uint8Array([KEY_CLICK_FLAGS, N4, SHIFT]));

const eight = document.getElementById("eight");
const six = document.getElementById("six");
const two = document.getElementById("two");
const four = document.getElementById("four");

createDownUpCustomButton("shift", () => {
    eight.innerHTML = "<span>↑</span>";
    six.innerHTML = "<span>→</span>";
    two.innerHTML = "<span>↓</span>";
    four.innerHTML = "<span>←</span>";
    createDownButtonElem(s, eight, new Uint8Array([KEY_CLICK_FLAGS, UP_ARROW, SHIFT]));
    createDownButtonElem(s, six, new Uint8Array([KEY_CLICK_FLAGS, RIGHT_ARROW, SHIFT]));
    createDownButtonElem(s, two, new Uint8Array([KEY_CLICK_FLAGS, DOWN_ARROW, SHIFT]));
    createDownButtonElem(s, four, new Uint8Array([KEY_CLICK_FLAGS, LEFT_ARROW, SHIFT]));
}, () => {
    eight.innerHTML = "<span>8</span>";
    six.innerHTML = "<span>6</span>";
    two.innerHTML = "<span>2</span>";
    four.innerHTML = "<span>4</span>";
    createDownButtonElem(s, eight, new Uint8Array([KEY_CLICK, N8]));
    createDownButtonElem(s, six, new Uint8Array([KEY_CLICK, N6]));
    createDownButtonElem(s, two, new Uint8Array([KEY_CLICK, N2]));
    createDownButtonElem(s, four, new Uint8Array([KEY_CLICK, N4]));
});