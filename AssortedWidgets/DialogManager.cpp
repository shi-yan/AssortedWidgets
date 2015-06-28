#include "DialogManager.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Manager
	{
		DialogManager::DialogManager(void):modalDialog(0)
		{
		}

		DialogManager::~DialogManager(void)
		{
		}

		void DialogManager::setModelessDialog(Widgets::Dialog *_modelessDialog)
		{
			std::vector<Widgets::Dialog*>::iterator iter;
			for(iter=modelessDialog.begin();iter<modelessDialog.end();++iter)
			{
				(*iter)->setActive(false);
			}
			modelessDialog.push_back(_modelessDialog);
			if(modalDialog)
			{
				_modelessDialog->setActive(false);
			}
			else
			{
				_modelessDialog->setActive(true);
			}
			_modelessDialog->setShowType(Widgets::Dialog::Modeless);
        }

		void DialogManager::setModalDialog(Widgets::Dialog *_modalDialog)
		{
			modalDialog=_modalDialog;
			modalDialog->setActive(true);
			modalDialog->setShowType(Widgets::Dialog::Modal);
			std::vector<Widgets::Dialog*>::iterator iter;
			for(iter=modelessDialog.begin();iter<modelessDialog.end();++iter)
			{
				(*iter)->setActive(false);
			}
        }

		void DialogManager::dropModalDialog()
		{
			modalDialog->setActive(false);
			modalDialog->setShowType(Widgets::Dialog::None);
			modalDialog=0;
			if(!modelessDialog.empty())
			{
				modelessDialog[modelessDialog.size()-1]->setActive(true);
			}
		}

		void DialogManager::dropModelessDialog(Widgets::Dialog *toBeDropped)
		{
			for(size_t i=0;i<modelessDialog.size();++i)
			{
				if(modelessDialog[i]==toBeDropped)
				{
					toBeDropped->setActive(false);
					toBeDropped->setShowType(Widgets::Dialog::None);
					modelessDialog[i]=modelessDialog[modelessDialog.size()-1];
					modelessDialog.pop_back();
				}
			}
		}

		void DialogManager::importMouseMotion(int mx,int my)
		{
			if(modalDialog)
			{
				if(modalDialog->isIn(mx,my))
				{
					if(modalDialog->isHover)
					{
						Event::MouseEvent event(modalDialog,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
						modalDialog->processMouseMoved(event);
					}
					else
					{
						Event::MouseEvent event(modalDialog,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
						modalDialog->processMouseEntered(event);
					}
					
				}
				else
				{
					if(modalDialog->isHover)
					{
						Event::MouseEvent event(modalDialog,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
						modalDialog->processMouseExited(event);
					}
				}
			}
			else
			{
				if(!modelessDialog.empty())
				{
					Widgets::Dialog *currentActive=modelessDialog[modelessDialog.size()-1];
					if(currentActive->isActive())
					{
						if(currentActive->isIn(mx,my))
						{
							if(currentActive->isHover)
							{
								Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
								currentActive->processMouseMoved(event);
							}
							else
							{
								Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
								currentActive->processMouseEntered(event);							
							}
						}
						else
						{
							if(currentActive->isHover)
							{
								Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
								currentActive->processMouseExited(event);
							}
						}
					}
				}
			}
		}

		void DialogManager::importMousePressed(int mx,int my)
		{
			if(modalDialog)
			{
				if(modalDialog->isIn(mx,my))
				{
					Event::MouseEvent event(modalDialog,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
					modalDialog->processMousePressed(event);
				}
			}
			else
			{
				if(!modelessDialog.empty())
				{
					Widgets::Dialog *currentActive=modelessDialog[modelessDialog.size()-1];
					if(currentActive->isActive())
					{
						if(currentActive->isIn(mx,my))
						{
							Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
							currentActive->processMousePressed(event);
						}
						else
						{
							for(int i=static_cast<int>(modelessDialog.size()-1);i>=0;--i)
							{
								if(modelessDialog[i]->isIn(mx,my))
								{
									modelessDialog[modelessDialog.size()-1]->setActive(false);
									modelessDialog[i]->setActive(true);

									Widgets::Dialog *temp(modelessDialog[i]);
									modelessDialog[i]=modelessDialog[modelessDialog.size()-1];
									modelessDialog[modelessDialog.size()-1]=temp;

									Event::MouseEvent event(temp,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
									temp->processMousePressed(event);

								}
							}
						}
					}
					else
					{
						for(int i=static_cast<int>(modelessDialog.size()-1);i>=0;--i)
						{
							if(modelessDialog[i]->isIn(mx,my))
							{
								modelessDialog[modelessDialog.size()-1]->setActive(false);
								modelessDialog[i]->setActive(true);

								Widgets::Dialog *temp(modelessDialog[i]);
								modelessDialog[i]=modelessDialog[modelessDialog.size()-1];
								modelessDialog[modelessDialog.size()-1]=temp;
							}
						}
					}
				}
			}
		}

		void DialogManager::importMouseReleased(int mx,int my)
		{
			if(modalDialog)
			{
				if(modalDialog->isIn(mx,my))
				{
					Event::MouseEvent event(modalDialog,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
					modalDialog->processMouseReleased(event);
				}
			}
			else
			{
				if(!modelessDialog.empty())
				{
					Widgets::Dialog *currentActive=modelessDialog[modelessDialog.size()-1];
					if(currentActive->isActive())
					{
						if(currentActive->isIn(mx,my))
						{
							Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
							currentActive->processMouseReleased(event);
						}
					}
				}
			}
		}

		void DialogManager::paint()
		{
			std::vector<Widgets::Dialog*>::iterator iter;
			for(iter=modelessDialog.begin();iter<modelessDialog.end();++iter)
			{
				(*iter)->paint();
			}
			if(modalDialog)
			{
				modalDialog->paint();
			}
		}
	}
}
