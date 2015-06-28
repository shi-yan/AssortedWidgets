#pragma once
#include "ThemeEngine.h"
#include "SubImage.h"


namespace AssortedWidgets
{
	namespace Theme
	{
		class DefaultTheme:public Theme
		{
		public:
			DefaultTheme(unsigned int _width,unsigned int _height);
		private:
            GLuint m_textureID;
            SubImage *m_MenuLeft;
            SubImage *m_MenuRight;
            SubImage *m_MenuListUpLeft;
            SubImage *m_MenuListUp;
            SubImage *m_MenuListUpRight;
            SubImage *m_MenuListLeft;
            SubImage *m_MenuListRight;
            SubImage *m_MenuListBottomLeft;
            SubImage *m_MenuListBottom;
            SubImage *m_MenuListBottomRight;
            SubImage *m_MenuItemSubMenuArrow;
            SubImage *m_ButtonNormalLeft;
            SubImage *m_ButtonNormalRight;
            SubImage *m_ButtonHoverLeft;
            SubImage *m_ButtonHoverRight;
            SubImage *m_RightHook;
            SubImage *m_RadioDot;
            SubImage *m_DialogUpLeftActive;
            SubImage *m_DialogUpLeftDeactive;
            SubImage *m_DialogUpActive;
            SubImage *m_DialogUpDeactive;
            SubImage *m_DialogUpRightActive;
            SubImage *m_DialogUpRightDeactive;
            SubImage *m_DialogLeft;
            SubImage *m_DialogRight;
            SubImage *m_DialogBottomLeft;
            SubImage *m_DialogBottom;
            SubImage *m_DialogBottomRight;
            SubImage *m_TextFieldLeft;
            SubImage *m_TextFieldRight;
            SubImage *m_Logo;
            SubImage *m_ScrollBarVerticalTopNormal;
            SubImage *m_ScrollBarVerticalBottomNormal;
            SubImage *m_ScrollBarHorizontalLeftNormal;
            SubImage *m_ScrollBarHorizontalRightNormal;
            SubImage *m_ScrollBarVerticalTopHover;
            SubImage *m_ScrollBarVerticalBottomHover;
            SubImage *m_ScrollBarHorizontalLeftHover;
            SubImage *m_ScrollBarHorizontalRightHover;
            SubImage *m_ScrollBarHorizontalBackground;
            SubImage *m_ScrollBarVerticalBackground;
            SubImage *m_CheckButtonOn;
            SubImage *m_CheckButtonOff;
            SubImage *m_RadioButtonOn;
            SubImage *m_RadioButtonOff;
            SubImage *m_ProgressBarLeft;
            SubImage *m_ProgressBarRight;
            SubImage *m_ProgressBarTop;
            SubImage *m_ProgressBarBottom;

		public:
			void setup();
			void uninstall()
			{				
                delete m_MenuLeft;
                delete m_MenuRight;
                delete m_MenuListUpLeft;
                delete m_MenuListUp;
                delete m_MenuListUpRight;
                delete m_MenuListLeft;
                delete m_MenuListRight;
                delete m_MenuListBottomLeft;
                delete m_MenuListBottom;
                delete m_MenuListBottomRight;
                delete m_MenuItemSubMenuArrow;
                delete m_ButtonNormalLeft;
                delete m_ButtonNormalRight;
                delete m_ButtonHoverLeft;
                delete m_ButtonHoverRight;
                delete m_RightHook;
                delete m_RadioDot;
                delete m_DialogUpLeftActive;
                delete m_DialogUpLeftDeactive;
                delete m_DialogUpActive;
                delete m_DialogUpDeactive;
                delete m_DialogUpRightActive;
                delete m_DialogUpRightDeactive;
                delete m_DialogLeft;
                delete m_DialogRight;
                delete m_DialogBottomLeft;
                delete m_DialogBottom;
                delete m_DialogBottomRight;
                delete m_TextFieldLeft;
                delete m_TextFieldRight;
                delete m_Logo;
                delete m_ScrollBarVerticalTopNormal;
                delete m_ScrollBarVerticalBottomNormal;
                delete m_ScrollBarHorizontalLeftNormal;
                delete m_ScrollBarHorizontalRightNormal;
                delete m_ScrollBarVerticalTopHover;
                delete m_ScrollBarVerticalBottomHover;
                delete m_ScrollBarHorizontalLeftHover;
                delete m_ScrollBarHorizontalRightHover;
                delete m_ScrollBarHorizontalBackground;
                delete m_ScrollBarVerticalBackground;
                delete m_CheckButtonOn;
                delete m_CheckButtonOff;
                delete m_RadioButtonOn;
                delete m_RadioButtonOff;
                delete m_ProgressBarLeft;
                delete m_ProgressBarRight;
                delete m_ProgressBarTop;
                delete m_ProgressBarBottom;

                glDeleteTextures(1, &m_textureID);
            }

			Util::Size getMenuPreferedSize(Widgets::Menu *component);
			void paintMenu(Widgets::Menu *component);

			Util::Size getMenuBarPreferedSize(Widgets::MenuBar *component);
			void paintMenuBar(Widgets::MenuBar *component);

			Util::Size getMenuListPreferedSize(Widgets::MenuList *component);
			void paintMenuList(Widgets::MenuList *component);

			Util::Size getMenuItemButtonPreferedSize(Widgets::MenuItemButton *component);
			void paintMenuItemButton(Widgets::MenuItemButton *component);

			Util::Size getMenuItemSeparatorPreferedSize(Widgets::MenuItemSeparator *component);
			void paintMenuItemSeparator(Widgets::MenuItemSeparator *component);

			Util::Size getMenuItemSubMenuPreferedSize(Widgets::MenuItemSubMenu *component);
			
			void paintMenuItemSubMenu(Widgets::MenuItemSubMenu *component);

            Util::Size getLabelPreferedSize(Widgets::Label *component) const;

			void paintLabel(Widgets::Label *component);

			Util::Size getButtonPreferedSize(Widgets::Button *component);
			
			void paintButton(Widgets::Button *component);

			Util::Size getMenuItemToggleButtonPreferedSize(Widgets::MenuItemToggleButton *component);

			void paintMenuItemToggleButton(Widgets::MenuItemToggleButton *component);

			Util::Size getMenuItemRadioButtonPreferedSize(Widgets::MenuItemRadioButton *component);

			void paintMenuItemRadioButton(Widgets::MenuItemRadioButton *component);
			
			Util::Size getMenuItemRadioGroupPreferedSize(Widgets::MenuItemRadioGroup *component);
			
			void paintMenuItemRadioGroup(Widgets::MenuItemRadioGroup *component);

			Util::Size getDialogPreferedSize(Widgets::Dialog *component);

			void paintDialog(Widgets::Dialog *component);

			Util::Size getDialogTittleBarPreferedSize(Widgets::DialogTittleBar *component);
			
			void paintDialogTittleBar(Widgets::DialogTittleBar *component);

			Util::Size getTextFieldPreferedSize(Widgets::TextField *component);

			void paintTextField(Widgets::TextField *component);

			Util::Size getLogoPreferedSize(Widgets::Logo *component);

			void paintLogo(Widgets::Logo *component);

			Util::Size getScrollBarButtonPreferedSize(Widgets::ScrollBarButton *component);
			void paintScrollBarButton(Widgets::ScrollBarButton *component);

			Util::Size getScrollBarSliderPreferedSize(Widgets::ScrollBarSlider *component);

			void paintScrollBarSlider(Widgets::ScrollBarSlider *component);

			Util::Size getScrollBarPreferedSize(Widgets::ScrollBar *component);
						
			void paintScrollBar(Widgets::ScrollBar *component);

			Util::Size getScrollPanelPreferedSize(Widgets::ScrollPanel *component);

			void paintScrollPanel(Widgets::ScrollPanel *component);

			void scissorBegin(Util::Position &position,Util::Size &area);

			void scissorEnd();

			Util::Size getCheckButtonPreferedSize(Widgets::CheckButton *component);

			void paintCheckButton(Widgets::CheckButton *component);

			Util::Size getRadioButtonPreferedSize(Widgets::RadioButton *component);

			void paintRadioButton(Widgets::RadioButton *component);

			Util::Size getProgressBarPreferedSize(Widgets::ProgressBar *component);

			void paintProgressBar(Widgets::ProgressBar *component);

			Util::Size getSlideBarSliderPreferedSize(Widgets::SlideBarSlider *component);

			void paintSlideBarSlider(Widgets::SlideBarSlider *component);

			Util::Size getSlideBarPreferedSize(Widgets::SlideBar *component);

			void paintSlideBar(Widgets::SlideBar *component);

			Util::Size getDropListButtonPreferedSize(Widgets::DropListButton *component);

			void paintDropListButton(Widgets::DropListButton *component);

			Util::Size getDropListPreferedSize(Widgets::DropList *component);

			void paintDropList(Widgets::DropList *component);

			Util::Size getDropListItemPreferedSize(Widgets::DropListItem *component);
			
			void paintDropListItem(Widgets::DropListItem *component);

			void paintDropDown(Util::Position &position,Util::Size &area);

			void test();
		public:
			DefaultTheme(void);
		public:
			~DefaultTheme(void);
		};
	}
}
