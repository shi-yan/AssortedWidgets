#include "SDL.h"
#include "SDL_opengl.h"
#include "UI.h"
#include <string>
#include <QApplication>
#include <windows.h>

void init(int width,int height)
{
	bool fullscreen =true;
	int bpp = 32;
	int flags = 0;

	if(SDL_Init(SDL_INIT_VIDEO) < 0)
	{
		printf("Video initialization failed: %s\n", SDL_GetError());
	}

	SDL_GL_SetAttribute(SDL_GL_RED_SIZE, 8);
	SDL_GL_SetAttribute(SDL_GL_GREEN_SIZE, 8);
	SDL_GL_SetAttribute(SDL_GL_BLUE_SIZE, 8);
	SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 32);
	SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);

	if(!fullscreen)
		flags = SDL_OPENGL;
	else
		flags = SDL_OPENGL | SDL_FULLSCREEN;
			
	if(SDL_SetVideoMode(width, height, bpp, flags) == 0)
	{
		printf("Video mode set failed: %s\n", SDL_GetError());
	}

	SDL_WM_SetCaption("Assorted Widgets",0);
	SDL_EnableUNICODE(1); 
	glShadeModel(GL_SMOOTH);
	glClearColor(118.0f/255.0f,130.0f/255.0f,123.0f/255.0f, 1.0f);
	glClearDepth(1.0f);
	glDepthFunc(GL_LEQUAL);
	glEnable(GL_DEPTH_TEST);
	glHint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST);

	glViewport(0,0,width,height);
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	glMatrixMode(GL_MODELVIEW);
}

void stop()
{
	SDL_QuitSubSystem(SDL_INIT_VIDEO);
}

void loop()
{
	bool out=false;
	while(!out)
	{
			int mx, my;
			SDL_GetMouseState(&mx,&my);
			AssortedWidgets::UI::getSingleton().mouseMotion(mx,my);

			SDL_Event event;
			while(SDL_PollEvent(&event))
			{
				switch(event.type)
				{
					case SDL_QUIT:
					{
						break;
					}
					case SDL_MOUSEBUTTONUP:
					{
						AssortedWidgets::UI::getSingleton().importMouseRelease(event.button.button,event.button.x,event.button.y);
						break;
					}
					case SDL_MOUSEBUTTONDOWN:
					{
						AssortedWidgets::UI::getSingleton().importMousePress(event.button.button,event.button.x,event.button.y);
						break;
					}
					case SDL_KEYDOWN:
					{
						AssortedWidgets::UI::getSingleton().importKeyDown(event.key.keysym.unicode,event.key.keysym.mod);
						break;
					}
					case SDL_KEYUP:
					{
						//out=true;
						AssortedWidgets::UI::getSingleton().importKeyUp(event.key.keysym.unicode,event.key.keysym.mod);
						break;
					}
				}
			}


		AssortedWidgets::UI::getSingleton().paint();
		SDL_GL_SwapBuffers();
	}
}

//int main(int argc, char* argv [])
int WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nShowCmd)
{
 //   QApplication application(argc, argv);
	int width=GetSystemMetrics(SM_CXSCREEN);
	int height=GetSystemMetrics(SM_CYSCREEN);
	init(width,height);
	AssortedWidgets::UI::getSingleton().init(width,height);
	//AssortedWidgets::UI::getSingleton().setQuitFunction(&stop);
	loop();
	stop();
	return 0;
}
