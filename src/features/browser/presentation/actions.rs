use gpui::actions;

actions!(
    browser,
    [
        MoveDown,
        MoveUp,
        SelectFirst,
        SelectLast,
        OpenEntry,
        GoParent,
        GoBack,
        GoForward,
        ToggleHidden,
        Reload,
    ]
);
