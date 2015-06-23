#pragma once
#include <vector>

namespace AssortedWidgets
{
	namespace Widgets
	{
		class Dialog;
	}
	namespace Manager
	{
		class DialogManager
		{
		private:
			Widgets::Dialog *modalDialog;
			std::vector<Widgets::Dialog*> modelessDialog;
		public:
			void setModalDialog(Widgets::Dialog *_modalDialog);
			void setModelessDialog(Widgets::Dialog *_modelessDialog);
			void dropModalDialog();
			void dropModelessDialog(Widgets::Dialog *toBeDropped);
			static DialogManager& getSingleton()
			{
				static DialogManager obj;
				return obj;
			};

			void importMouseMotion(int mx,int my);
			void importMousePressed(int mx,int my);
			void importMouseReleased(int mx,int my);
			void paint();
		private:
			DialogManager(void);
			~DialogManager(void);
		};
	}
}