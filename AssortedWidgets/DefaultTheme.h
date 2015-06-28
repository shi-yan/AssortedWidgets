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
			GLuint textureID;
			SubImage *MenuLeft;
			SubImage *MenuRight;
			SubImage *MenuListUpLeft;
			SubImage *MenuListUp;
			SubImage *MenuListUpRight;
			SubImage *MenuListLeft;
			SubImage *MenuListRight;
			SubImage *MenuListBottomLeft;
			SubImage *MenuListBottom;
			SubImage *MenuListBottomRight;
			SubImage *MenuItemSubMenuArrow;
			SubImage *ButtonNormalLeft;
			SubImage *ButtonNormalRight;
			SubImage *ButtonHoverLeft;
			SubImage *ButtonHoverRight;
			SubImage *RightHook;
			SubImage *RadioDot;
			SubImage *DialogUpLeftActive;
			SubImage *DialogUpLeftDeactive;
			SubImage *DialogUpActive;
			SubImage *DialogUpDeactive;
			SubImage *DialogUpRightActive;
			SubImage *DialogUpRightDeactive;
			SubImage *DialogLeft;
			SubImage *DialogRight;
			SubImage *DialogBottomLeft;
			SubImage *DialogBottom;
			SubImage *DialogBottomRight;
			SubImage *TextFieldLeft;
			SubImage *TextFieldRight;
			SubImage *Logo;
			SubImage *ScrollBarVerticalTopNormal;
			SubImage *ScrollBarVerticalBottomNormal;
			SubImage *ScrollBarHorizontalLeftNormal;
			SubImage *ScrollBarHorizontalRightNormal;
			SubImage *ScrollBarVerticalTopHover;
			SubImage *ScrollBarVerticalBottomHover;
			SubImage *ScrollBarHorizontalLeftHover;
			SubImage *ScrollBarHorizontalRightHover;
			SubImage *ScrollBarHorizontalBackground;
			SubImage *ScrollBarVerticalBackground;
			SubImage *CheckButtonOn;
			SubImage *CheckButtonOff;
			SubImage *RadioButtonOn;
			SubImage *RadioButtonOff;
			SubImage *ProgressBarLeft;
			SubImage *ProgressBarRight;
			SubImage *ProgressBarTop;
			SubImage *ProgressBarBottom;

		public:
			void setup();
			void uninstall()
			{				
				delete MenuLeft;
				delete MenuRight;
				delete MenuListUpLeft;
				delete MenuListUp;
				delete MenuListUpRight;
				delete MenuListLeft;
				delete MenuListRight;
				delete MenuListBottomLeft;
				delete MenuListBottom;
				delete MenuListBottomRight;
				delete MenuItemSubMenuArrow;
				delete ButtonNormalLeft;
				delete ButtonNormalRight;
				delete ButtonHoverLeft;
				delete ButtonHoverRight;
				delete RightHook;
				delete RadioDot;
				delete DialogUpLeftActive;
				delete DialogUpLeftDeactive;
				delete DialogUpActive;
				delete DialogUpDeactive;
				delete DialogUpRightActive;
				delete DialogUpRightDeactive;
				delete DialogLeft;
				delete DialogRight;
				delete DialogBottomLeft;
				delete DialogBottom;
				delete DialogBottomRight;
				delete TextFieldLeft;
				delete TextFieldRight;
				delete Logo;
				delete ScrollBarVerticalTopNormal;
				delete ScrollBarVerticalBottomNormal;
				delete ScrollBarHorizontalLeftNormal;
				delete ScrollBarHorizontalRightNormal;
				delete ScrollBarVerticalTopHover;
				delete ScrollBarVerticalBottomHover;
				delete ScrollBarHorizontalLeftHover;
				delete ScrollBarHorizontalRightHover;
				delete ScrollBarHorizontalBackground;
				delete ScrollBarVerticalBackground;
				delete CheckButtonOn;
				delete CheckButtonOff;
				delete RadioButtonOn;
				delete RadioButtonOff;
				delete ProgressBarLeft;
				delete ProgressBarRight;
				delete ProgressBarTop;
				delete ProgressBarBottom;

				glDeleteTextures(1,&textureID);
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
