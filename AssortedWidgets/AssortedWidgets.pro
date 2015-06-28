QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = AssortedWidgets
TEMPLATE = app

CONFIG -= console
CONFIG += C++14


#INCLUDEPATH += C:/temp/freetype-2.6/include \
#               C:/temp/SDL-1.2.15/include \
#               C:/temp/SDL_image-1.2.12

#LIBS += -LC:/temp/SDL_image-1.2.12/VisualC/Debug \
#        -LC:/temp/SDL-1.2.15/lib/x86 \
#        -LC:/temp/freetype-2.6/objs/vc2010/Win32 \
#        -LC:/Qt/Qt5.4.2/5.4/msvc2013_opengl/lib \
#        -lSDL \
#        -lfreetype26d \
#        -lSDL_image \
#        -lqtmaind

INCLUDEPATH += /usr/include/freetype2 \

LIBS += -lSDL2 -lGL -lGLU -lSDL2_image -lfreetype

HEADERS += \
    AbstractButton.h \
    AllInOneDialog.h \
    BorderLayout.h \
    BorderLayoutTestDialog.h \
    BoundingBox.h \
    Button.h \
    CheckButton.h \
    CheckNRadioTestDialog.h \
    Component.h \
    ContainerElement.h \
    DefaultTheme.h \
    Dialog.h \
    DialogBottom.h \
    DialogBottomLeft.h \
    DialogBottomRight.h \
    DialogLeft.h \
    DialogManager.h \
    DialogRight.h \
    DialogTestDialog.h \
    DialogTittleBar.h \
    DialogUp.h \
    DialogUpLeft.h \
    DialogUpRight.h \
    DragAble.h \
    DragManager.h \
    DropList.h \
    DropListButton.h \
    DropListItem.h \
    DropListManager.h \
    Event.h \
    FlowLayout.h \
    FlowLayoutTestDialog.h \
    Font.h \
    FontEngine.h \
    FreeTypeFont.h \
    GirdLayout.h \
    GirdLayoutTestDialog.h \
    Graphics.h \
    KeyEvent.h \
    Label.h \
    LabelNButtonTestDialog.h \
    Layout.h \
    Logo.h \
    Menu.h \
    MenuBar.h \
    MenuItem.h \
    MenuItemButton.h \
    MenuItemRadioButton.h \
    MenuItemRadioGroup.h \
    MenuItemSeparator.h \
    MenuItemSpacer.h \
    MenuItemSubMenu.h \
    MenuItemToggleButton.h \
    MenuList.h \
    MenuTheme.h \
    MouseEvent.h \
    MouseListener.h \
    MultipleLayoutTestDialog.h \
    OGLFT.h \
    Panel.h \
    PanelTestDialog.h \
    Position.h \
    ProgressBar.h \
    ProgressNSliderTestDialog.h \
    RadioButton.h \
    RadioGroup.h \
    ScrollBar.h \
    ScrollBarButton.h \
    ScrollBarSlider.h \
    ScrollPanel.h \
    SelectionManager.h \
    Size.h \
    SlideBar.h \
    SlideBarSlider.h \
    Spacer.h \
    SubImage.h \
    TextField.h \
    TextNDropTestDialog.h \
    Theme.h \
    ThemeEngine.h \
    TypeAble.h \
    TypeActiveManager.h \
    UI.h

SOURCES += \
    AbstractButton.cpp \
    AllInOneDialog.cpp \
    BorderLayout.cpp \
    BorderLayoutTestDialog.cpp \
    Button.cpp \
    CheckButton.cpp \
    CheckNRadioTestDialog.cpp \
    Component.cpp \
    DefaultTheme.cpp \
    Dialog.cpp \
    DialogBottom.cpp \
    DialogBottomLeft.cpp \
    DialogBottomRight.cpp \
    DialogLeft.cpp \
    DialogManager.cpp \
    DialogRight.cpp \
    DialogTestDialog.cpp \
    DialogTittleBar.cpp \
    DialogUp.cpp \
    DialogUpLeft.cpp \
    DialogUpRight.cpp \
    DragAble.cpp \
    DropList.cpp \
    DropListButton.cpp \
    DropListItem.cpp \
    DropListManager.cpp \
    FlowLayout.cpp \
    FlowLayoutTestDialog.cpp \
    Font.cpp \
    FontEngine.cpp \
    FreeTypeFont.cpp \
    GirdLayout.cpp \
    GirdLayoutTestDialog.cpp \
    Label.cpp \
    LabelNButtonTestDialog.cpp \
    Logo.cpp \
    Main.cpp \
    Menu.cpp \
    MenuBar.cpp \
    MenuItem.cpp \
    MenuItemButton.cpp \
    MenuItemRadioButton.cpp \
    MenuItemRadioGroup.cpp \
    MenuItemSeparator.cpp \
    MenuItemSpacer.cpp \
    MenuItemSubMenu.cpp \
    MenuItemToggleButton.cpp \
    MenuList.cpp \
    MultipleLayoutTestDialog.cpp \
    OGLFT.cpp \
    Panel.cpp \
    PanelTestDialog.cpp \
    ProgressBar.cpp \
    ProgressNSliderTestDialog.cpp \
    RadioButton.cpp \
    RadioGroup.cpp \
    ScrollBar.cpp \
    ScrollBarButton.cpp \
    ScrollBarSlider.cpp \
    ScrollPanel.cpp \
    SelectionManager.cpp \
    SlideBar.cpp \
    SlideBarSlider.cpp \
    Spacer.cpp \
    TextField.cpp \
    TextNDropTestDialog.cpp \
    TypeAble.cpp \
    TypeActiveManager.cpp \
    UI.cpp
