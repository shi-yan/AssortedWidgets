#pragma once

#include "Size.h"
#include "Position.h"

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Menu;
		class MenuBar;
		class MenuList;
		class MenuItemButton;
		class MenuItemSeparator;
		class MenuItemSubMenu;
		class MenuItemToggleButton;
		class MenuItemRadioButton;
		class MenuItemRadioGroup;
		class Label;
		class Button;
		class Dialog;
		class DialogTittleBar;
		class TextField;
		class Logo;
		class ScrollBarButton;
		class ScrollBarSlider;
		class ScrollBar;
		class ScrollPanel;
		class CheckButton;
		class RadioButton;
		class ProgressBar;
		class SlideBarSlider;
		class SlideBar;
		class DropListButton;
		class DropList;
		class DropListItem;
	}

	namespace Theme
	{
		class Theme
		{
		protected:
			unsigned int screenWidth;
			unsigned int screenHeight;

		public:
			virtual void setup()=0;
			virtual void test()=0;
			virtual void uninstall()=0;
			virtual Util::Size getMenuPreferedSize(Widgets::Menu *component)=0;
			virtual void paintMenu(Widgets::Menu *component)=0;
			virtual Util::Size getMenuBarPreferedSize(Widgets::MenuBar *component)=0;
			virtual void paintMenuBar(Widgets::MenuBar *component)=0;
			virtual Util::Size getMenuListPreferedSize(Widgets::MenuList *component)=0;
			virtual void paintMenuList(Widgets::MenuList *component)=0;
			virtual Util::Size getMenuItemButtonPreferedSize(Widgets::MenuItemButton *component)=0;
			virtual void paintMenuItemButton(Widgets::MenuItemButton *component)=0;
			virtual Util::Size getMenuItemSeparatorPreferedSize(Widgets::MenuItemSeparator *component)=0;
			virtual void paintMenuItemSeparator(Widgets::MenuItemSeparator *component)=0;
			virtual Util::Size getMenuItemSubMenuPreferedSize(Widgets::MenuItemSubMenu *component)=0;
			virtual void paintMenuItemSubMenu(Widgets::MenuItemSubMenu *component)=0;
			virtual Util::Size getLabelPreferedSize(Widgets::Label *component)=0;
			virtual void paintLabel(Widgets::Label *component)=0;
			virtual Util::Size getButtonPreferedSize(Widgets::Button *component)=0;
			virtual void paintButton(Widgets::Button *component)=0;
			virtual Util::Size getMenuItemToggleButtonPreferedSize(Widgets::MenuItemToggleButton *component)=0;
			virtual void paintMenuItemToggleButton(Widgets::MenuItemToggleButton *component)=0;
			virtual Util::Size getMenuItemRadioButtonPreferedSize(Widgets::MenuItemRadioButton *component)=0;
			virtual void paintMenuItemRadioButton(Widgets::MenuItemRadioButton *component)=0;
			virtual Util::Size getMenuItemRadioGroupPreferedSize(Widgets::MenuItemRadioGroup *component)=0;
			virtual void paintMenuItemRadioGroup(Widgets::MenuItemRadioGroup *component)=0;
			virtual Util::Size getDialogPreferedSize(Widgets::Dialog *component)=0;
			virtual void paintDialog(Widgets::Dialog *component)=0;
			virtual Util::Size getDialogTittleBarPreferedSize(Widgets::DialogTittleBar *component)=0;
			virtual void paintDialogTittleBar(Widgets::DialogTittleBar *component)=0;
			virtual Util::Size getTextFieldPreferedSize(Widgets::TextField *component)=0;
			virtual void paintTextField(Widgets::TextField *component)=0;
			virtual Util::Size getLogoPreferedSize(Widgets::Logo *component)=0;
			virtual void paintLogo(Widgets::Logo *component)=0;
			virtual Util::Size getScrollBarButtonPreferedSize(Widgets::ScrollBarButton *component)=0;
			virtual void paintScrollBarButton(Widgets::ScrollBarButton *component)=0;
			virtual Util::Size getScrollBarSliderPreferedSize(Widgets::ScrollBarSlider *component)=0;
			virtual void paintScrollBarSlider(Widgets::ScrollBarSlider *component)=0;
			virtual Util::Size getScrollBarPreferedSize(Widgets::ScrollBar *component)=0;
			virtual void paintScrollBar(Widgets::ScrollBar *component)=0;
			virtual Util::Size getScrollPanelPreferedSize(Widgets::ScrollPanel *component)=0;
			virtual void paintScrollPanel(Widgets::ScrollPanel *component)=0;
			virtual Util::Size getCheckButtonPreferedSize(Widgets::CheckButton *component)=0;
			virtual void paintCheckButton(Widgets::CheckButton *component)=0;
			virtual Util::Size getRadioButtonPreferedSize(Widgets::RadioButton *component)=0;
			virtual void paintRadioButton(Widgets::RadioButton *component)=0;
			virtual Util::Size getProgressBarPreferedSize(Widgets::ProgressBar *component)=0;
			virtual void paintProgressBar(Widgets::ProgressBar *component)=0;
			virtual Util::Size getSlideBarSliderPreferedSize(Widgets::SlideBarSlider *component)=0;
			virtual void paintSlideBarSlider(Widgets::SlideBarSlider *component)=0;
			virtual Util::Size getSlideBarPreferedSize(Widgets::SlideBar *component)=0;
			virtual void paintSlideBar(Widgets::SlideBar *component)=0;
			virtual Util::Size getDropListButtonPreferedSize(Widgets::DropListButton *component)=0;
			virtual void paintDropListButton(Widgets::DropListButton *component)=0;
			virtual Util::Size getDropListPreferedSize(Widgets::DropList *component)=0;
			virtual void paintDropList(Widgets::DropList *component)=0;

			virtual Util::Size getDropListItemPreferedSize(Widgets::DropListItem *component)=0;
			virtual void paintDropListItem(Widgets::DropListItem *component)=0;
			virtual void paintDropDown(Util::Position &position,Util::Size &area)=0;
			virtual void scissorBegin(Util::Position &position,Util::Size &area)=0;
			virtual void scissorEnd()=0;
		};

		class ThemeEngine
		{
		private:
			Theme *theme;
		private:
			ThemeEngine():theme(0)
			{};
		public:
			void setupTheme(Theme *_theme)
			{
				if(theme)
				{
					delete theme;
				}
				theme=_theme;
			}
			static ThemeEngine& getSingleton()
			{
				static ThemeEngine obj;
				return obj;
			};
			Theme& getTheme() const
			{
				return *theme;
			};
		private:
			~ThemeEngine()
			{
				if(theme)
				{
					delete theme;
				}
			};
		};
	}
}