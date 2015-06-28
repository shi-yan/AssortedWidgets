#include "DialogManager.h"
#include "Dialog.h"

namespace AssortedWidgets
{
	namespace Manager
	{
        DialogManager::DialogManager(void)
            :m_modalDialog(0)
		{
		}

		DialogManager::~DialogManager(void)
		{
		}

		void DialogManager::setModelessDialog(Widgets::Dialog *_modelessDialog)
		{
			std::vector<Widgets::Dialog*>::iterator iter;
            for(iter=m_modelessDialog.begin();iter<m_modelessDialog.end();++iter)
			{
				(*iter)->setActive(false);
			}
            m_modelessDialog.push_back(_modelessDialog);
            if(m_modalDialog)
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
            m_modalDialog=_modalDialog;
            m_modalDialog->setActive(true);
            m_modalDialog->setShowType(Widgets::Dialog::Modal);
			std::vector<Widgets::Dialog*>::iterator iter;
            for(iter=m_modelessDialog.begin();iter<m_modelessDialog.end();++iter)
			{
				(*iter)->setActive(false);
			}
        }

		void DialogManager::dropModalDialog()
		{
            m_modalDialog->setActive(false);
            m_modalDialog->setShowType(Widgets::Dialog::None);
            m_modalDialog=0;
            if(!m_modelessDialog.empty())
			{
                m_modelessDialog[m_modelessDialog.size()-1]->setActive(true);
			}
		}

		void DialogManager::dropModelessDialog(Widgets::Dialog *toBeDropped)
		{
            for(size_t i=0;i<m_modelessDialog.size();++i)
			{
                if(m_modelessDialog[i]==toBeDropped)
				{
					toBeDropped->setActive(false);
					toBeDropped->setShowType(Widgets::Dialog::None);
                    m_modelessDialog[i]=m_modelessDialog[m_modelessDialog.size()-1];
                    m_modelessDialog.pop_back();
				}
			}
		}

		void DialogManager::importMouseMotion(int mx,int my)
		{
            if(m_modalDialog)
			{
                if(m_modalDialog->isIn(mx,my))
				{
                    if(m_modalDialog->m_isHover)
					{
                        Event::MouseEvent event(m_modalDialog,Event::MouseEvent::MOUSE_MOTION,mx,my,0);
                        m_modalDialog->processMouseMoved(event);
					}
					else
					{
                        Event::MouseEvent event(m_modalDialog,Event::MouseEvent::MOUSE_ENTERED,mx,my,0);
                        m_modalDialog->processMouseEntered(event);
					}
					
				}
				else
				{
                    if(m_modalDialog->m_isHover)
					{
                        Event::MouseEvent event(m_modalDialog,Event::MouseEvent::MOUSE_EXITED,mx,my,0);
                        m_modalDialog->processMouseExited(event);
					}
				}
			}
			else
			{
                if(!m_modelessDialog.empty())
				{
                    Widgets::Dialog *currentActive = m_modelessDialog[m_modelessDialog.size()-1];
					if(currentActive->isActive())
					{
						if(currentActive->isIn(mx,my))
						{
                            if(currentActive->m_isHover)
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
                            if(currentActive->m_isHover)
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
            if(m_modalDialog)
			{
                if(m_modalDialog->isIn(mx,my))
				{
                    Event::MouseEvent event(m_modalDialog,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
                    m_modalDialog->processMousePressed(event);
				}
			}
			else
			{
                if(!m_modelessDialog.empty())
				{
                    Widgets::Dialog *currentActive=m_modelessDialog[m_modelessDialog.size()-1];
					if(currentActive->isActive())
					{
						if(currentActive->isIn(mx,my))
						{
							Event::MouseEvent event(currentActive,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
							currentActive->processMousePressed(event);
						}
						else
						{
                            for(int i=static_cast<int>(m_modelessDialog.size()-1);i>=0;--i)
							{
                                if(m_modelessDialog[i]->isIn(mx,my))
								{
                                    m_modelessDialog[m_modelessDialog.size()-1]->setActive(false);
                                    m_modelessDialog[i]->setActive(true);

                                    Widgets::Dialog *temp(m_modelessDialog[i]);
                                    m_modelessDialog[i]=m_modelessDialog[m_modelessDialog.size()-1];
                                    m_modelessDialog[m_modelessDialog.size()-1]=temp;

									Event::MouseEvent event(temp,Event::MouseEvent::MOUSE_PRESSED,mx,my,0);
									temp->processMousePressed(event);

								}
							}
						}
					}
					else
					{
                        for(int i=static_cast<int>(m_modelessDialog.size()-1);i>=0;--i)
						{
                            if(m_modelessDialog[i]->isIn(mx,my))
							{
                                m_modelessDialog[m_modelessDialog.size()-1]->setActive(false);
                                m_modelessDialog[i]->setActive(true);

                                Widgets::Dialog *temp(m_modelessDialog[i]);
                                m_modelessDialog[i]=m_modelessDialog[m_modelessDialog.size()-1];
                                m_modelessDialog[m_modelessDialog.size()-1]=temp;
							}
						}
					}
				}
			}
		}

		void DialogManager::importMouseReleased(int mx,int my)
		{
            if(m_modalDialog)
			{
                if(m_modalDialog->isIn(mx,my))
				{
                    Event::MouseEvent event(m_modalDialog,Event::MouseEvent::MOUSE_RELEASED,mx,my,0);
                    m_modalDialog->processMouseReleased(event);
				}
			}
			else
			{
                if(!m_modelessDialog.empty())
				{
                    Widgets::Dialog *currentActive=m_modelessDialog[m_modelessDialog.size()-1];
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
            for(iter=m_modelessDialog.begin();iter<m_modelessDialog.end();++iter)
			{
				(*iter)->paint();
			}
            if(m_modalDialog)
			{
                m_modalDialog->paint();
			}
		}
	}
}
