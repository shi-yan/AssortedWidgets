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
macx {
INCLUDEPATH += /usr/local/Cellar/sdl2/2.0.4/include \
               /usr/local/Cellar/sdl2_image/2.0.1_1/include \
               /usr/local/Cellar/sdl2/2.0.4/include/SDL2


LIBS += -L/usr/local/Cellar/sdl2/2.0.4/lib \
        -L/usr/local/Cellar/sdl2_image/2.0.1_1/lib
LIBS += -framework OpenGL -lSDL2 -lSDL2_image

}

unix:!mac {
LIBS += -lSDL2 -lGLESv2 -lSDL2_image
}

HEADERS += \
    AbstractButton.h \
    BorderLayout.h \
    BoundingBox.h \
    Button.h \
    CheckButton.h \
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
    Font.h \
    FontEngine.h \
    GridLayout.h \
    Graphics.h \
    KeyEvent.h \
    Label.h \
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
    Panel.h \
    Position.h \
    ProgressBar.h \
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
    Theme.h \
    ThemeEngine.h \
    TypeAble.h \
    TypeActiveManager.h \
    UI.h \
    fontstash.h \
    stb_truetype.h \
    TrueTypeFont.h \
    DialogTitleBar.h \
    GraphicsBackend.h \
    ../demo/AllInOneDialog.h \
    ../demo/BorderLayoutTestDialog.h \
    ../demo/CheckNRadioTestDialog.h \
    ../demo/DialogTestDialog.h \
    ../demo/FlowLayoutTestDialog.h \
    ../demo/GridLayoutTestDialog.h \
    ../demo/LabelNButtonTestDialog.h \
    ../demo/MultipleLayoutTestDialog.h \
    ../demo/PanelTestDialog.h \
    ../demo/ProgressNSliderTestDialog.h \
    ../demo/TextNDropTestDialog.h

SOURCES += \
    AbstractButton.cpp \
    BorderLayout.cpp \
    Button.cpp \
    CheckButton.cpp \
    Component.cpp \
    DefaultTheme.cpp \
    Dialog.cpp \
    DialogBottom.cpp \
    DialogBottomLeft.cpp \
    DialogBottomRight.cpp \
    DialogLeft.cpp \
    DialogManager.cpp \
    DialogRight.cpp \
    DialogUp.cpp \
    DialogUpLeft.cpp \
    DialogUpRight.cpp \
    DragAble.cpp \
    DropList.cpp \
    DropListButton.cpp \
    DropListItem.cpp \
    DropListManager.cpp \
    FlowLayout.cpp \
    Font.cpp \
    FontEngine.cpp \
    GridLayout.cpp \
    Label.cpp \
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
    Panel.cpp \
    ProgressBar.cpp \
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
    TypeAble.cpp \
    TypeActiveManager.cpp \
    UI.cpp \
    TrueTypeFont.cpp \
    DialogTitleBar.cpp \
    SubImage.cpp \
    GraphicsBackend.cpp \
    ../demo/AllInOneDialog.cpp \
    ../demo/BorderLayoutTestDialog.cpp \
    ../demo/CheckNRadioTestDialog.cpp \
    ../demo/DialogTestDialog.cpp \
    ../demo/FlowLayoutTestDialog.cpp \
    ../demo/GridLayoutTestDialog.cpp \
    ../demo/LabelNButtonTestDialog.cpp \
    ../demo/MultipleLayoutTestDialog.cpp \
    ../demo/PanelTestDialog.cpp \
    ../demo/ProgressNSliderTestDialog.cpp \
    ../demo/TextNDropTestDialog.cpp


INCLUDEPATH += /usr/include/freetype2

macx {
    RESOURCEFILE.files     = aw.png \
                            arial.ttf

    RESOURCEFILE.path      = Contents/MacOS

    QMAKE_BUNDLE_DATA += RESOURCEFILE
}
