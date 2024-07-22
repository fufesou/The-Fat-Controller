enumeration!(
    CommandCode,
    "The discriminant of [`Command`](crate::Command) (useful for serialization).",
    [
        (Delay, "Delay"),
        (KeyDown, "Key Down"),
        (KeyUp, "Key Up"),
        (KeyClick, "Key Click"),
        (MouseMoveRel, "Mouse Move Relative"),
        (MouseMoveAbs, "Mouse Move Absolute"),
        (MouseScroll, "Mouse Scroll"),
        (MouseDown, "Mouse Down"),
        (MouseUp, "Mouse Up"),
        (MouseClick, "Mouse Click"),
        (AsciiCharDown, "ASCII Character Down"),
        (AsciiCharUp, "ASCII Character Up"),
        (AsciiChar, "ASCII Character"),
        (AsciiString, "ASCII String"),
        (UnicodeCharDown, "Unicode Character Down"),
        (UnicodeCharUp, "Unicode Character Up"),
        (UnicodeChar, "Unicode Character"),
        (UnicodeString, "Unicode String"),
    ]
);
