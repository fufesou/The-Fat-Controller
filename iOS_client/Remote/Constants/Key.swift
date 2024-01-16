// This file was generated automatically

enum Key: UInt8, CaseIterable {
    case capsLock
    case shift
    case control
    case alt
    case meta
    case controlOrMeta
    case rightShift
    case rightControl
    case rightAlt
    case rightMeta
    case rightControlOrMeta
    case fn
    case returnOrEnter
    case escape
    case deleteOrBackspace
    case forwardDelete
    case insert
    case tab
    case space
    case minus
    case equal
    case leftBracket
    case rightBracket
    case backslash
    case semicolon
    case quote
    case grave
    case comma
    case period
    case slash
    case upArrow
    case rightArrow
    case downArrow
    case leftArrow
    case pageUp
    case pageDown
    case home
    case end
    case a
    case b
    case c
    case d
    case e
    case f
    case g
    case h
    case i
    case j
    case k
    case l
    case m
    case n
    case o
    case p
    case q
    case r
    case s
    case t
    case u
    case v
    case w
    case x
    case y
    case z
    case n0
    case n1
    case n2
    case n3
    case n4
    case n5
    case n6
    case n7
    case n8
    case n9
    case numpad0
    case numpad1
    case numpad2
    case numpad3
    case numpad4
    case numpad5
    case numpad6
    case numpad7
    case numpad8
    case numpad9
    case numpadClear
    case numpadEquals
    case numpadDivide
    case numpadMultiply
    case numpadMinus
    case numpadPlus
    case numpadEnter
    case numpadDecimal
    case f1
    case f2
    case f3
    case f4
    case f5
    case f6
    case f7
    case f8
    case f9
    case f10
    case f11
    case f12
    case fastForward
    case rewind
    case playPause
    case volumeUp
    case volumeDown
    case mute
}

extension Key: CustomStringConvertible {
    var description: String {
        switch self {
            case .capsLock: return "Caps Lock"
            case .shift: return "Shift"
            case .control: return "Control"
            case .alt: return "Alt"
            case .meta: return "Meta"
            case .controlOrMeta: return "Control or Meta"
            case .rightShift: return "Right Shift"
            case .rightControl: return "Right Control"
            case .rightAlt: return "Right Alt"
            case .rightMeta: return "Right Meta"
            case .rightControlOrMeta: return "Right Control or Meta"
            case .fn: return "Fn"
            case .returnOrEnter: return "Return or Enter"
            case .escape: return "Escape"
            case .deleteOrBackspace: return "Delete or Backspace"
            case .forwardDelete: return "Forward Delete"
            case .insert: return "Insert"
            case .tab: return "Tab"
            case .space: return "Space"
            case .minus: return "Minus"
            case .equal: return "Equal"
            case .leftBracket: return "Left Bracket"
            case .rightBracket: return "Right Bracket"
            case .backslash: return "Backslash"
            case .semicolon: return "Semicolon"
            case .quote: return "Quote"
            case .grave: return "Grave"
            case .comma: return "Comma"
            case .period: return "Period"
            case .slash: return "Slash"
            case .upArrow: return "Up Arrow"
            case .rightArrow: return "Right Arrow"
            case .downArrow: return "Down Arrow"
            case .leftArrow: return "Left Arrow"
            case .pageUp: return "Page Up"
            case .pageDown: return "Page Down"
            case .home: return "Home"
            case .end: return "End"
            case .a: return "A"
            case .b: return "B"
            case .c: return "C"
            case .d: return "D"
            case .e: return "E"
            case .f: return "F"
            case .g: return "G"
            case .h: return "H"
            case .i: return "I"
            case .j: return "J"
            case .k: return "K"
            case .l: return "L"
            case .m: return "M"
            case .n: return "N"
            case .o: return "O"
            case .p: return "P"
            case .q: return "Q"
            case .r: return "R"
            case .s: return "S"
            case .t: return "T"
            case .u: return "U"
            case .v: return "V"
            case .w: return "W"
            case .x: return "X"
            case .y: return "Y"
            case .z: return "Z"
            case .n0: return "0"
            case .n1: return "1"
            case .n2: return "2"
            case .n3: return "3"
            case .n4: return "4"
            case .n5: return "5"
            case .n6: return "6"
            case .n7: return "7"
            case .n8: return "8"
            case .n9: return "9"
            case .numpad0: return "Numpad 0"
            case .numpad1: return "Numpad 1"
            case .numpad2: return "Numpad 2"
            case .numpad3: return "Numpad 3"
            case .numpad4: return "Numpad 4"
            case .numpad5: return "Numpad 5"
            case .numpad6: return "Numpad 6"
            case .numpad7: return "Numpad 7"
            case .numpad8: return "Numpad 8"
            case .numpad9: return "Numpad 9"
            case .numpadClear: return "Numpad Clear"
            case .numpadEquals: return "Numpad Equals"
            case .numpadDivide: return "Numpad Divide"
            case .numpadMultiply: return "Numpad Multiply"
            case .numpadMinus: return "Numpad Minus"
            case .numpadPlus: return "Numpad Plus"
            case .numpadEnter: return "Numpad Enter"
            case .numpadDecimal: return "Numpad Decimal"
            case .f1: return "F1"
            case .f2: return "F2"
            case .f3: return "F3"
            case .f4: return "F4"
            case .f5: return "F5"
            case .f6: return "F6"
            case .f7: return "F7"
            case .f8: return "F8"
            case .f9: return "F9"
            case .f10: return "F10"
            case .f11: return "F11"
            case .f12: return "F12"
            case .fastForward: return "Fast-Forward"
            case .rewind: return "Rewind"
            case .playPause: return "Play/Pause"
            case .volumeUp: return "Volume Up"
            case .volumeDown: return "Volume Down"
            case .mute: return "Mute"
        }
    }
}

extension Key: Enum {}
