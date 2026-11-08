use gpui::actions;

actions!(
    shell,
    [
        ToggleRail,
        FocusRail,
        RailNext,
        RailPrevious,
        RailOpen,
        RailLeave,
        RailEject,
        NewWindow,
        DetachTab
    ]
);
